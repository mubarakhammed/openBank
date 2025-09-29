use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use tracing::{info, warn};

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window_size: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
            window_size: Duration::from_secs(60),
        }
    }
}

/// Rate limiter state for a single IP
#[derive(Debug, Clone)]
struct RateLimitState {
    requests: Vec<Instant>,
    blocked_until: Option<Instant>,
}

/// In-memory rate limiter (use Redis in production for distributed systems)
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: RateLimitConfig,
    states: Arc<Mutex<HashMap<String, RateLimitState>>>,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_rate_limit(&self, ip: &str) -> Result<(), RateLimitError> {
        let mut states = self.states.lock().unwrap();
        let now = Instant::now();

        let state = states.entry(ip.to_string()).or_insert(RateLimitState {
            requests: Vec::new(),
            blocked_until: None,
        });

        // Check if still blocked
        if let Some(blocked_until) = state.blocked_until {
            if now < blocked_until {
                return Err(RateLimitError::Blocked {
                    retry_after: blocked_until.duration_since(now),
                });
            } else {
                state.blocked_until = None;
            }
        }

        // Clean old requests outside the window
        state
            .requests
            .retain(|&request_time| now.duration_since(request_time) < self.config.window_size);

        // Check if exceeding rate limit
        if state.requests.len() >= self.config.requests_per_minute as usize {
            let block_duration = Duration::from_secs(300); // 5 minutes
            state.blocked_until = Some(now + block_duration);

            warn!(
                ip = ip,
                requests_count = state.requests.len(),
                "Rate limit exceeded, blocking IP"
            );

            return Err(RateLimitError::ExceededLimit {
                requests_made: state.requests.len() as u32,
                limit: self.config.requests_per_minute,
                retry_after: block_duration,
            });
        }

        // Check burst limit
        let recent_requests = state
            .requests
            .iter()
            .filter(|&&request_time| now.duration_since(request_time) < Duration::from_secs(10))
            .count();

        if recent_requests >= self.config.burst_size as usize {
            return Err(RateLimitError::BurstExceeded {
                burst_count: recent_requests as u32,
                burst_limit: self.config.burst_size,
            });
        }

        // Record this request
        state.requests.push(now);

        info!(
            ip = ip,
            requests_in_window = state.requests.len(),
            "Rate limit check passed"
        );

        Ok(())
    }

    /// Clean up expired entries (call periodically)
    pub fn cleanup_expired(&self) {
        let mut states = self.states.lock().unwrap();
        let now = Instant::now();

        states.retain(|_ip, state| {
            // Keep if blocked or has recent requests
            state.blocked_until.is_some()
                || !state.requests.is_empty()
                || state
                    .requests
                    .iter()
                    .any(|&request_time| now.duration_since(request_time) < self.config.window_size)
        });
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded: {requests_made}/{limit} requests. Retry after {retry_after:?}")]
    ExceededLimit {
        requests_made: u32,
        limit: u32,
        retry_after: Duration,
    },
    #[error("Burst limit exceeded: {burst_count}/{burst_limit} requests in 10 seconds")]
    BurstExceeded { burst_count: u32, burst_limit: u32 },
    #[error("IP blocked. Retry after {retry_after:?}")]
    Blocked { retry_after: Duration },
}

/// Rate limiting middleware
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    use axum::http::StatusCode;

    // Extract rate limiter from app state (implement this based on your app structure)
    // For now, using default config
    let rate_limiter = RateLimiter::new(RateLimitConfig::default());

    let ip = addr.ip().to_string();

    match rate_limiter.check_rate_limit(&ip) {
        Ok(()) => {
            // Rate limit passed, continue to next middleware/handler
            Ok(next.run(req).await)
        }
        Err(RateLimitError::ExceededLimit { retry_after, .. }) => {
            warn!(ip = ip, "Rate limit exceeded");

            // Add retry-after header
            let mut response = Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body("Rate limit exceeded".into())
                .unwrap();

            response.headers_mut().insert(
                "Retry-After",
                retry_after.as_secs().to_string().parse().unwrap(),
            );

            Ok(response)
        }
        Err(RateLimitError::BurstExceeded { .. }) => {
            warn!(ip = ip, "Burst limit exceeded");
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
        Err(RateLimitError::Blocked { retry_after }) => {
            warn!(ip = ip, "IP blocked due to previous violations");

            let mut response = Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body("IP temporarily blocked".into())
                .unwrap();

            response.headers_mut().insert(
                "Retry-After",
                retry_after.as_secs().to_string().parse().unwrap(),
            );

            Ok(response)
        }
    }
}

/// Helper to extract client IP from various headers
pub fn extract_client_ip(req: &Request) -> String {
    // Check X-Forwarded-For first (for load balancers/proxies)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }

    // Check X-Real-IP
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.to_string();
        }
    }

    // Fallback to connection info
    "unknown".to_string()
}

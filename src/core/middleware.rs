use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn};
use crate::core::{
    AppState,
    audit::{AuditEvent, AuditEventType, AuditSeverity, extract_audit_context},
    rate_limit::RateLimitError,
};

/// Combined security middleware that handles rate limiting, audit logging, and monitoring
pub async fn security_middleware(
    State(app_state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    let start_time = Instant::now();
    
    // Extract request context for audit logging
    let audit_context = extract_audit_context(&req);
    
    // 1. Rate Limiting Check
    match app_state.rate_limiter.check_rate_limit(&audit_context.ip_address) {
        Ok(()) => {
            // Rate limit passed
            info!(
                ip = audit_context.ip_address,
                method = audit_context.method,
                uri = audit_context.uri,
                "Request passed rate limiting"
            );
        }
        Err(RateLimitError::ExceededLimit { requests_made, limit, retry_after }) => {
            warn!(
                ip = audit_context.ip_address,
                requests_made = requests_made,
                limit = limit,
                "Rate limit exceeded"
            );

            // Log rate limit violation
            let event = AuditEvent::new(AuditEventType::RateLimitExceeded)
                .severity(AuditSeverity::Warning)
                .ip_address(audit_context.ip_address.clone())
                .user_agent(audit_context.user_agent.clone().unwrap_or_default())
                .metadata("requests_made".to_string(), serde_json::to_value(requests_made).unwrap())
                .metadata("limit".to_string(), serde_json::to_value(limit).unwrap())
                .risk_score(40)
                .compliance_tag("SECURITY".to_string());

            app_state.audit_logger.log(event).await;

            let response = Response::builder()
                .status(axum::http::StatusCode::TOO_MANY_REQUESTS)
                .header("Retry-After", retry_after.as_secs().to_string())
                .header("X-RateLimit-Limit", limit.to_string())
                .header("X-RateLimit-Remaining", "0")
                .body("Rate limit exceeded. Please try again later.".into())
                .unwrap();

            return Ok(response);
        }
        Err(RateLimitError::BurstExceeded { burst_count, burst_limit }) => {
            warn!(
                ip = audit_context.ip_address,
                burst_count = burst_count,
                burst_limit = burst_limit,
                "Burst limit exceeded"
            );

            let event = AuditEvent::new(AuditEventType::RateLimitExceeded)
                .severity(AuditSeverity::Warning)
                .ip_address(audit_context.ip_address.clone())
                .user_agent(audit_context.user_agent.clone().unwrap_or_default())
                .metadata("burst_count".to_string(), serde_json::to_value(burst_count).unwrap())
                .metadata("burst_limit".to_string(), serde_json::to_value(burst_limit).unwrap())
                .risk_score(35)
                .compliance_tag("SECURITY".to_string());

            app_state.audit_logger.log(event).await;

            return Err(axum::http::StatusCode::TOO_MANY_REQUESTS);
        }
        Err(RateLimitError::Blocked { retry_after }) => {
            warn!(
                ip = audit_context.ip_address,
                "IP blocked due to previous violations"
            );

            let event = AuditEvent::new(AuditEventType::AccessDenied)
                .severity(AuditSeverity::Critical)
                .ip_address(audit_context.ip_address.clone())
                .user_agent(audit_context.user_agent.clone().unwrap_or_default())
                .error("IP temporarily blocked due to security violations".to_string())
                .risk_score(80)
                .compliance_tag("SECURITY".to_string());

            app_state.audit_logger.log(event).await;

            let response = Response::builder()
                .status(axum::http::StatusCode::TOO_MANY_REQUESTS)
                .header("Retry-After", retry_after.as_secs().to_string())
                .body("IP temporarily blocked. Please try again later.".into())
                .unwrap();

            return Ok(response);
        }
    }

    // 2. Add request ID for tracing
    let request_id = uuid::Uuid::new_v4().to_string();
    req.headers_mut().insert(
        "x-request-id",
        request_id.clone().parse().unwrap(),
    );

    // 3. Process request
    let response = next.run(req).await;
    let duration = start_time.elapsed();

        // 4. Log successful request completion
    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;
    let success = response.status().is_success();
    
    // Clone audit context fields before using them
    let audit_ip = audit_context.ip_address.clone();
    let audit_user_agent = audit_context.user_agent.clone();
    let audit_uri = audit_context.uri.clone();
    let audit_method = audit_context.method.clone();
    
    let event = AuditEvent::new(AuditEventType::ApiAccess)
        .severity(if success { AuditSeverity::Info } else { AuditSeverity::Warning })
        .ip_address(audit_ip)
        .user_agent(audit_user_agent.unwrap_or_default())
        .request_id(request_id.clone())
        .success(success)
        .resource(audit_uri.clone())
        .action(audit_method.clone())
        .metadata("duration_ms".to_string(), serde_json::to_value(duration.as_millis()).unwrap())
        .metadata("status_code".to_string(), serde_json::to_value(response.status().as_u16()).unwrap())
        .risk_score(if success { 0 } else { 20 })
        .compliance_tag("API_ACCESS".to_string());

    app_state.audit_logger.log(event).await;

    // 5. Add security headers to response
    let mut response = response;
    let headers = response.headers_mut();
    
    // Security headers
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin".parse().unwrap());
    headers.insert("X-Request-ID", request_id.clone().parse().unwrap());

    // Performance monitoring
    if app_state.config.performance_monitoring_enabled {
        headers.insert("X-Response-Time", format!("{}ms", duration.as_millis()).parse().unwrap());
    }

    info!(
        request_id = %request_id,
        method = %audit_method,
        uri = %audit_uri,
        status = response.status().as_u16(),
        duration_ms = duration.as_millis(),
        "Request completed"
    );

    Ok(response)
}

/// Authentication security middleware for checking account lockout and suspicious activity
pub async fn auth_security_middleware(
    State(app_state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // This middleware would be applied specifically to auth endpoints
    // to check for account lockout, suspicious activity, etc.
    
    let audit_context = extract_audit_context(&req);
    let is_oauth = req.uri().path().starts_with("/oauth");
    
    // For now, just pass through - the actual security checks will be 
    // implemented in the auth service integration
    let response = next.run(req).await;
    
    // Log authentication-related events
    if is_oauth {
        let event = AuditEvent::new(AuditEventType::ApiAccess)
            .severity(AuditSeverity::Info)
            .ip_address(audit_context.ip_address.clone())
            .user_agent(audit_context.user_agent.clone().unwrap_or_default())
            .resource("oauth_endpoint".to_string())
            .action(audit_context.method.clone())
            .success(response.status().is_success())
            .compliance_tag("OAUTH2".to_string())
            .compliance_tag("AUTHENTICATION".to_string());

        app_state.audit_logger.log(event).await;
    }
    
    Ok(response)
}

/// RBAC middleware for checking permissions
pub async fn rbac_middleware(
    State(app_state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    // Extract JWT claims (if present)
    // Check if user has required permissions for the endpoint
    // This would be endpoint-specific and require integration with JWT middleware
    
    let audit_context = extract_audit_context(&req);
    let is_api = req.uri().path().starts_with("/api");
    let resource_path = req.uri().path().to_string();
    
    // For now, pass through - full RBAC integration requires JWT token extraction
    let response = next.run(req).await;
    
    // Log authorization events for protected endpoints
    if is_api {
        let event = AuditEvent::new(AuditEventType::AccessGranted)
            .severity(AuditSeverity::Info)
            .ip_address(audit_context.ip_address.clone())
            .user_agent(audit_context.user_agent.clone().unwrap_or_default())
            .resource(resource_path)
            .action(audit_context.method.clone())
            .success(response.status().is_success())
            .compliance_tag("RBAC".to_string())
            .compliance_tag("AUTHORIZATION".to_string());

        app_state.audit_logger.log(event).await;
    }
    
    Ok(response)
}
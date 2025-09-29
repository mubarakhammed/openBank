use axum::{response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod core;
mod shared;

// Module declarations
mod auth;
mod identity;
mod income;
mod payments;
mod transactions;
mod user_data;
mod virtual_accounts;

use core::config::Config;
use core::database::init_mongodb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "openbank=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env().map_err(|e| e as Box<dyn std::error::Error>)?;
    info!("Configuration loaded successfully");

    // Initialize databases (skip migrations for testing)
    let postgres_pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            info!("PostgreSQL connection established successfully");
            pool
        }
        Err(e) => {
            tracing::warn!("Failed to connect to PostgreSQL: {}. Running in development mode without database.", e);
            // For now, we'll exit. In production, you might want to handle this differently
            return Err(e.into());
        }
    };

    let mongodb_client = match init_mongodb(&config.mongodb_url).await {
        Ok(client) => {
            info!("MongoDB connection established successfully");
            client
        }
        Err(e) => {
            tracing::warn!(
                "Failed to connect to MongoDB: {}. Running without analytics.",
                e
            );
            // For now, we'll exit. In production, you might want to handle this differently
            return Err(e.into());
        }
    };

    info!("Database connections established");

    // Initialize Audit Logger with separate MongoDB connection
    let audit_mongodb_client = match init_mongodb(&config.mongodb_audit_url).await {
        Ok(client) => {
            info!("MongoDB audit connection established successfully");
            client
        }
        Err(e) => {
            tracing::warn!(
                "Failed to connect to MongoDB audit: {}. Using main MongoDB.",
                e
            );
            mongodb_client.clone()
        }
    };

    // Initialize Security Services
    let audit_logger = core::audit::AuditLogger::new(audit_mongodb_client);
    let security_config = core::security::SecurityConfig {
        max_failed_attempts: config.max_failed_attempts,
        lockout_duration_minutes: config.account_lockout_duration_minutes,
        progressive_lockout: config.progressive_lockout_enabled,
        suspicious_activity_threshold: config.suspicious_activity_threshold,
        password_history_count: config.password_history_count,
        require_password_change_days: config.require_password_change_days,
    };
    let security_service = core::security::AccountSecurityService::new(security_config);
    let rbac_service = core::rbac::RbacService::new();
    let rate_limit_config = core::rate_limit::RateLimitConfig {
        requests_per_minute: config.rate_limit_requests_per_minute as u32,
        burst_size: config.rate_limit_burst_size,
        window_size: std::time::Duration::from_secs(config.rate_limit_window_seconds),
    };
    let rate_limiter = core::rate_limit::RateLimiter::new(rate_limit_config);

    info!("Security services initialized");

    // Create Auth service for OAuth2 API-as-a-Service
    let auth_service = auth::service::AuthService::new(
        auth::repository::AuthRepository::new(postgres_pool.clone()),
        config.jwt_secret.clone(),
    );

    // Create AppState with all services
    let app_state = core::AppState {
        postgres: postgres_pool,
        mongodb: mongodb_client,
        config: config.clone(),
        audit_logger,
        security_service,
        rbac_service,
        rate_limiter,
    };

    // Build our application with routes and security middleware
    let fintech_app = Router::new()
        .route("/health", get(health_check))
        // Legacy fintech routes (with state)
        .nest("/api/v1/user-data", user_data::routes())
        .nest("/api/v1/identity", identity::routes())
        .nest("/api/v1/income", income::routes())
        .nest("/api/v1/payments", payments::routes())
        .nest("/api/v1/transactions", transactions::routes())
        .nest("/api/v1/virtual-accounts", virtual_accounts::routes())
        .with_state(app_state.clone());

    // Merge OAuth2 routes (no state) with fintech routes (with state)
    let app = fintech_app
        .merge(auth::routes(auth_service.clone()))
        // Security middleware layers (applied in reverse order)
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            core::middleware::rbac_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            core::middleware::auth_security_middleware,
        ))
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            core::middleware::security_middleware,
        ))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    info!("Server starting on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct HealthData {
    service: String,
    version: String,
    timestamp: String,
}

async fn health_check() -> Json<core::response::ApiResponse<HealthData>> {
    let health_data = HealthData {
        service: "openBank".to_string(),
        version: "0.1.0".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Json(core::response::ApiResponse::success(
        "Service is healthy and operational",
        health_data,
    ))
}

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::{json, Value};
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

    // Create Auth service for OAuth2 API-as-a-Service
    let auth_service = auth::service::AuthService::new(
        auth::repository::AuthRepository::new(postgres_pool.clone()),
        config.jwt_secret.clone(),
    );

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        // OAuth2 API-as-a-Service Auth routes
        .nest("/oauth", auth::routes(auth_service.clone()))
        .with_state(())
        // Legacy fintech routes (keep existing functionality)
        .nest("/api/v1/user-data", user_data::routes())
        .nest("/api/v1/identity", identity::routes())
        .nest("/api/v1/income", income::routes())
        .nest("/api/v1/payments", payments::routes())
        .nest("/api/v1/transactions", transactions::routes())
        .nest("/api/v1/virtual-accounts", virtual_accounts::routes())
        .layer(CorsLayer::permissive())
        .with_state(core::AppState {
            postgres: postgres_pool,
            mongodb: mongodb_client,
            config,
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    info!("Server starting on http://127.0.0.1:8080");

    axum::serve(listener, app)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(())
}

async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "service": "openBank",
        "version": "0.1.0"
    })))
}

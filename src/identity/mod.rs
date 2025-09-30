pub mod controller;
pub mod handlers;
pub mod ml_service;
pub mod model;
pub mod repository_new;
pub mod service;

use anyhow::Result;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::identity::handlers::IdentityHandlers;
use crate::identity::model::ModelConfig;
use crate::identity::repository_new::IdentityRepository;
use crate::identity::service::IdentityService;

/// Initialize and create the identity service with all dependencies
pub async fn create_identity_service(pool: PgPool) -> Result<IdentityService> {
    // Create model configuration
    let config = ModelConfig {
        face_detection_model_path: "models/face_detection.onnx".to_string(),
        face_recognition_model_path: "models/face_recognition.onnx".to_string(),
        liveness_model_path: "models/liveness.onnx".to_string(),
        anti_spoof_model_path: "models/anti_spoof.onnx".to_string(),
        model_version: "v1.0.0".to_string(),
        input_size: (112, 112),
        embedding_size: 512,
    };

    // Create repository
    let repository = IdentityRepository::new(pool);

    // Create service
    let service = IdentityService::new(repository, config).await?;

    // Initialize database schema
    service.initialize().await?;

    Ok(service)
}

/// Create identity API routes compatible with AppState
pub fn create_routes_for_app_state() -> Router<crate::core::AppState> {
    use axum::{routing::get, Router};

    // Create basic identity routes that will work with AppState
    Router::new().route("/identity/health", get(identity_health_handler))
    // Additional routes can be added here when properly integrated
}

/// Simple health check handler for identity service
async fn identity_health_handler() -> &'static str {
    "Identity service operational"
}

/// Create identity API routes with dedicated service state
pub fn create_routes(service: IdentityService) -> Router<Arc<IdentityService>> {
    let handlers = IdentityHandlers::new(service.clone());
    handlers.routes().with_state(Arc::new(service))
}

/// Main routes function for external use
pub fn routes() -> Router<crate::core::AppState> {
    create_routes_for_app_state()
}

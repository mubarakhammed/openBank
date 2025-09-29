use super::response::{ApiResponse, ErrorResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Application-wide error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::MongoDB(err) => {
                tracing::error!("MongoDB error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "MongoDB error")
            }
            AppError::Validation(ref msg) => {
                tracing::warn!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, "Validation error")
            }
            AppError::Authentication(ref msg) => {
                tracing::warn!("Authentication error: {}", msg);
                (StatusCode::UNAUTHORIZED, "Authentication error")
            }
            AppError::Authorization(ref msg) => {
                tracing::warn!("Authorization error: {}", msg);
                (StatusCode::FORBIDDEN, "Authorization error")
            }
            AppError::NotFound(ref msg) => {
                tracing::info!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, "Not found")
            }
            AppError::Conflict(ref msg) => {
                tracing::warn!("Conflict: {}", msg);
                (StatusCode::CONFLICT, "Conflict")
            }
            AppError::BadRequest(ref msg) => {
                tracing::warn!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, "Bad request")
            }
            AppError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::ExternalService(ref msg) => {
                tracing::error!("External service error: {}", msg);
                (StatusCode::BAD_GATEWAY, "External service error")
            }
        };

        let error_code = match &self {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::MongoDB(_) => "MONGODB_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Authentication(_) => "AUTHENTICATION_ERROR",
            AppError::Authorization(_) => "AUTHORIZATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
        };

        let response =
            ApiResponse::<ErrorResponse>::error("Request failed", error_code, error_message);

        (status, Json(response)).into_response()
    }
}

/// Result type alias for the application
pub type AppResult<T> = Result<T, AppError>;

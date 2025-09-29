use super::model::*;
use super::service::AuthService;
use crate::core::error::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use validator::Validate;

pub fn routes(auth_service: AuthService) -> Router {
    Router::new()
        .route("/developers", post(register_developer))
        .route("/token", post(oauth_token))
        .route("/developers/:developer_id/projects", post(create_project))
        .route("/me", get(get_me))
        .with_state(auth_service)
}

pub async fn register_developer(
    State(service): State<AuthService>,
    Json(request): Json<RegisterDeveloperRequest>,
) -> Result<(StatusCode, Json<DeveloperResponse>), (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_errors) = request.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: "Invalid request data".to_string(),
                details: Some(serde_json::to_value(validation_errors).unwrap_or_default()),
            }),
        ));
    }

    match service.register_developer(request).await {
        Ok(developer) => Ok((StatusCode::CREATED, Json(developer))),
        Err(error) => {
            let (status_code, error_code, message) = match error {
                AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg),
                AppError::Authentication(msg) => {
                    (StatusCode::UNAUTHORIZED, "authentication_error", msg)
                }
                AppError::Authorization(msg) => (StatusCode::FORBIDDEN, "authorization_error", msg),
                AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
                AppError::Internal(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
                }
                AppError::Database(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    err.to_string(),
                ),
                AppError::MongoDB(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "mongodb_error",
                    err.to_string(),
                ),
                AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict_error", msg),
                AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
                AppError::ExternalService(msg) => {
                    (StatusCode::BAD_GATEWAY, "external_service_error", msg)
                }
            };

            Err((
                status_code,
                Json(ErrorResponse {
                    error: error_code.to_string(),
                    message,
                    details: None,
                }),
            ))
        }
    }
}

pub async fn oauth_token(
    State(service): State<AuthService>,
    Json(request): Json<TokenRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_errors) = request.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: "Invalid request data".to_string(),
                details: Some(serde_json::to_value(validation_errors).unwrap_or_default()),
            }),
        ));
    }

    match service.handle_client_credentials_flow(request).await {
        Ok(token) => Ok((StatusCode::OK, Json(token))),
        Err(error) => {
            let (status_code, error_code, message) = match error {
                AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg),
                AppError::Authentication(msg) => {
                    (StatusCode::UNAUTHORIZED, "authentication_error", msg)
                }
                AppError::Authorization(msg) => (StatusCode::FORBIDDEN, "authorization_error", msg),
                AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
                AppError::Internal(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
                }
                AppError::Database(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    err.to_string(),
                ),
                AppError::MongoDB(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "mongodb_error",
                    err.to_string(),
                ),
                AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict_error", msg),
                AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
                AppError::ExternalService(msg) => {
                    (StatusCode::BAD_GATEWAY, "external_service_error", msg)
                }
            };

            Err((
                status_code,
                Json(ErrorResponse {
                    error: error_code.to_string(),
                    message,
                    details: None,
                }),
            ))
        }
    }
}

pub async fn create_project(
    State(service): State<AuthService>,
    Path(developer_id): Path<uuid::Uuid>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ProjectResponse>), (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_errors) = request.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "validation_error".to_string(),
                message: "Invalid request data".to_string(),
                details: Some(serde_json::to_value(validation_errors).unwrap_or_default()),
            }),
        ));
    }

    match service.create_project(developer_id, request).await {
        Ok(project) => Ok((StatusCode::CREATED, Json(project))),
        Err(error) => {
            let (status_code, error_code, message) = match error {
                AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "validation_error", msg),
                AppError::Authentication(msg) => {
                    (StatusCode::UNAUTHORIZED, "authentication_error", msg)
                }
                AppError::Authorization(msg) => (StatusCode::FORBIDDEN, "authorization_error", msg),
                AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
                AppError::Internal(msg) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg)
                }
                AppError::Database(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    err.to_string(),
                ),
                AppError::MongoDB(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "mongodb_error",
                    err.to_string(),
                ),
                AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict_error", msg),
                AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
                AppError::ExternalService(msg) => {
                    (StatusCode::BAD_GATEWAY, "external_service_error", msg)
                }
            };

            Err((
                status_code,
                Json(ErrorResponse {
                    error: error_code.to_string(),
                    message,
                    details: None,
                }),
            ))
        }
    }
}

pub async fn get_me(
    State(service): State<AuthService>,
    // TODO: Extract JWT token from Authorization header
) -> Result<(StatusCode, Json<MeResponse>), (StatusCode, Json<ErrorResponse>)> {
    // This is a placeholder - you would extract the JWT token from the Authorization header
    // and verify it using service.verify_access_token()
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ErrorResponse {
            error: "not_implemented".to_string(),
            message: "JWT token extraction not implemented yet".to_string(),
            details: None,
        }),
    ))
}

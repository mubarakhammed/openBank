use super::model::*;
use super::service::AuthService;
use crate::core::error::AppError;
use crate::core::extractors::ApiJson;
use crate::core::response::ApiResponse;
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
    ApiJson(request): ApiJson<RegisterDeveloperRequest>,
) -> Result<(StatusCode, Json<ApiResponse<DeveloperResponse>>), AppError> {
    if let Err(validation_errors) = request.validate() {
        return Err(AppError::Validation(format!(
            "Invalid request data: {:?}",
            validation_errors
        )));
    }

    match service.register_developer(request).await {
        Ok(developer) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::success(
                "Developer registered successfully",
                developer,
            )),
        )),
        Err(error) => Err(error),
    }
}

pub async fn oauth_token(
    State(service): State<AuthService>,
    ApiJson(request): ApiJson<TokenRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, AppError> {
    if let Err(validation_errors) = request.validate() {
        return Err(AppError::Validation(format!(
            "Invalid request data: {:?}",
            validation_errors
        )));
    }

    match service.handle_client_credentials_flow(request).await {
        Ok(token) => Ok(Json(ApiResponse::success(
            "Access token generated successfully",
            token,
        ))),
        Err(error) => Err(error),
    }
}

pub async fn create_project(
    State(service): State<AuthService>,
    Path(developer_id): Path<uuid::Uuid>,
    ApiJson(request): ApiJson<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ProjectResponse>>), AppError> {
    if let Err(validation_errors) = request.validate() {
        return Err(AppError::Validation(format!(
            "Invalid request data: {:?}",
            validation_errors
        )));
    }

    match service.create_project(developer_id, request).await {
        Ok(project) => Ok((
            StatusCode::CREATED,
            Json(ApiResponse::success(
                "Project created successfully",
                project,
            )),
        )),
        Err(error) => Err(error),
    }
}

pub async fn get_me(
    State(_service): State<AuthService>,
    // TODO: Extract JWT token from Authorization header
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    // This is a placeholder - you would extract the JWT token from the Authorization header
    // and verify it using service.verify_access_token()
    Err(AppError::Internal(
        "JWT token extraction not implemented yet".to_string(),
    ))
}

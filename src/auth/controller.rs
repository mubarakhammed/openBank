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
        .route("/token/refresh", post(refresh_token))
        .route("/developers/:developer_id/projects", post(create_project))
        .route("/me", get(get_me))
        .route("/scopes", get(get_available_scopes))
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

pub async fn refresh_token(
    State(service): State<AuthService>,
    ApiJson(request): ApiJson<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<TokenResponse>>, AppError> {
    if let Err(validation_errors) = request.validate() {
        return Err(AppError::Validation(format!(
            "Invalid request data: {:?}",
            validation_errors
        )));
    }

    match service.refresh_access_token(request).await {
        Ok(token) => Ok(Json(ApiResponse::success(
            "Access token refreshed successfully",
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
    State(service): State<AuthService>,
    headers: axum::http::HeaderMap,
) -> Result<Json<ApiResponse<MeResponse>>, AppError> {
    // Extract Authorization header
    let auth_header = headers
        .get("authorization")
        .ok_or_else(|| AppError::Authentication("Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| AppError::Authentication("Invalid Authorization header".to_string()))?;

    // Check if it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Authentication(
            "Authorization header must start with 'Bearer '".to_string(),
        ));
    }

    // Extract the token
    let token = auth_header.strip_prefix("Bearer ").unwrap();

    // Verify the token using the service
    match service.verify_access_token(token).await {
        Ok(me_response) => Ok(Json(ApiResponse::success(
            "Token verified successfully",
            me_response,
        ))),
        Err(error) => Err(error),
    }
}

pub async fn get_available_scopes() -> Json<ApiResponse<ScopesResponse>> {
    use crate::auth::scopes;

    let scopes_with_descriptions: Vec<ScopeInfo> = scopes::all_scopes()
        .into_iter()
        .map(|scope| ScopeInfo {
            scope: scope.clone(),
            description: scopes::get_scope_description(&scope)
                .unwrap_or("No description available")
                .to_string(),
        })
        .collect();

    let response = ScopesResponse {
        scopes: scopes_with_descriptions,
        scope_sets: ScopeSetsInfo {
            basic: scopes::ScopeSets::basic(),
            banking_app: scopes::ScopeSets::banking_app(),
            fintech_platform: scopes::ScopeSets::fintech_platform(),
            identity_service: scopes::ScopeSets::identity_service(),
            income_service: scopes::ScopeSets::income_service(),
            full_access: scopes::ScopeSets::full_access(),
        },
    };

    Json(ApiResponse::success(
        "Available scopes retrieved successfully",
        response,
    ))
}

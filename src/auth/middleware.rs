use crate::auth::model::JwtClaims;
use crate::core::error::AppError;
use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

/// JWT token extractor for protected routes
pub struct JwtToken(pub JwtClaims);

#[async_trait]
impl<S> FromRequestParts<S> for JwtToken
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
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
        let _token = auth_header.strip_prefix("Bearer ").unwrap();

        // This is a placeholder - we need access to the JWT secret
        // We'll handle this with dependency injection in the middleware
        Err(AppError::Authentication(
            "Token verification requires middleware setup".to_string(),
        ))
    }
}

/// JWT Authentication middleware
pub async fn jwt_auth_middleware(
    mut req: Request,
    next: Next,
    jwt_secret: String,
) -> Result<Response, StatusCode> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract the token
    let token = auth_header.strip_prefix("Bearer ").unwrap();

    // Verify the token
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add the claims to request extensions for use in handlers
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

/// Helper to extract JWT claims from request extensions
pub fn extract_claims(req: &Request) -> Option<&JwtClaims> {
    req.extensions().get::<JwtClaims>()
}

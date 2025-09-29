use crate::core::error::AppError;
use crate::core::response::ApiResponse;
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;

/// Custom JSON extractor that provides better error messages
pub struct ApiJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ApiJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(ApiJson(value)),
            Err(rejection) => {
                let error_message = match rejection {
                    axum::extract::rejection::JsonRejection::JsonDataError(err) => {
                        format!("Invalid JSON data: {}", err)
                    }
                    axum::extract::rejection::JsonRejection::JsonSyntaxError(err) => {
                        format!("Invalid JSON syntax: {}", err)
                    }
                    axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                        "Missing or invalid Content-Type header. Expected 'application/json'"
                            .to_string()
                    }
                    axum::extract::rejection::JsonRejection::BytesRejection(err) => {
                        format!("Failed to read request body: {}", err)
                    }
                    _ => "Invalid JSON request".to_string(),
                };

                Err(AppError::BadRequest(error_message))
            }
        }
    }
}

impl<T> std::ops::Deref for ApiJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ApiJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

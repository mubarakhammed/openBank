use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard API response wrapper for all OpenBank endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Response status: "success", "error", "pending"
    pub status: ResponseStatus,
    /// Human-readable message describing the response
    pub message: String,
    /// The actual response data (optional for error responses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Additional metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Standard error response for API errors
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code for programmatic handling
    pub error_code: String,
    /// Human-readable error message
    pub error_message: String,
    /// Additional error details (validation errors, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Response status enumeration
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
    Pending,
}

impl<T> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            status: ResponseStatus::Success,
            message: message.into(),
            data: Some(data),
            meta: None,
        }
    }

    /// Create a successful response with data and metadata
    pub fn success_with_meta(
        message: impl Into<String>, 
        data: T, 
        meta: HashMap<String, serde_json::Value>
    ) -> Self {
        Self {
            status: ResponseStatus::Success,
            message: message.into(),
            data: Some(data),
            meta: Some(meta),
        }
    }

    /// Create a pending response (for async operations)
    pub fn pending(message: impl Into<String>, data: T) -> Self {
        Self {
            status: ResponseStatus::Pending,
            message: message.into(),
            data: Some(data),
            meta: None,
        }
    }
}

impl ApiResponse<()> {
    /// Create a successful response without data
    pub fn success_no_data(message: impl Into<String>) -> Self {
        Self {
            status: ResponseStatus::Success,
            message: message.into(),
            data: None,
            meta: None,
        }
    }
}

impl ApiResponse<ErrorResponse> {
    /// Create an error response
    pub fn error(
        message: impl Into<String>,
        error_code: impl Into<String>,
        error_message: impl Into<String>,
    ) -> Self {
        Self {
            status: ResponseStatus::Error,
            message: message.into(),
            data: Some(ErrorResponse {
                error_code: error_code.into(),
                error_message: error_message.into(),
                details: None,
            }),
            meta: None,
        }
    }

    /// Create an error response with details
    pub fn error_with_details(
        message: impl Into<String>,
        error_code: impl Into<String>,
        error_message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            status: ResponseStatus::Error,
            message: message.into(),
            data: Some(ErrorResponse {
                error_code: error_code.into(),
                error_message: error_message.into(),
                details: Some(details),
            }),
            meta: None,
        }
    }
}

/// Trait for converting domain objects to API responses
pub trait IntoApiResponse<T> {
    fn into_success_response(self, message: impl Into<String>) -> ApiResponse<T>;
}

// Implement for common types
impl<T> IntoApiResponse<T> for T {
    fn into_success_response(self, message: impl Into<String>) -> ApiResponse<T> {
        ApiResponse::success(message, self)
    }
}
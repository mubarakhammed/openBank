use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common timestamp fields for entities
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timestamps {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            message: None,
            error: Some(error),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

/// User ID type alias
pub type UserId = Uuid;

/// Account ID type alias
pub type AccountId = Uuid;

/// Transaction ID type alias
pub type TransactionId = Uuid;

/// Money amount type (in cents to avoid floating point issues)
pub type Amount = i64;

/// Currency code (ISO 4217)
pub type Currency = String;
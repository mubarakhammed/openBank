use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use crate::shared::types::{AccountId, UserId, Currency};

/// Virtual account status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "virtual_account_status", rename_all = "lowercase")]
pub enum VirtualAccountStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

/// Virtual account model for database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VirtualAccount {
    pub id: Uuid,
    pub user_id: UserId,
    pub parent_account_id: AccountId,
    pub account_number: String,
    pub account_name: String,
    pub currency: Currency,
    pub status: VirtualAccountStatus,
    pub purpose: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create virtual account request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateVirtualAccountRequest {
    pub parent_account_id: AccountId,
    #[validate(length(min = 1))]
    pub account_name: String,
    pub currency: Currency,
    pub purpose: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Virtual account response
#[derive(Debug, Serialize)]
pub struct VirtualAccountResponse {
    pub id: Uuid,
    pub account_number: String,
    pub account_name: String,
    pub currency: Currency,
    pub status: VirtualAccountStatus,
    pub purpose: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<VirtualAccount> for VirtualAccountResponse {
    fn from(account: VirtualAccount) -> Self {
        Self {
            id: account.id,
            account_number: account.account_number,
            account_name: account.account_name,
            currency: account.currency,
            status: account.status,
            purpose: account.purpose,
            created_at: account.created_at,
        }
    }
}
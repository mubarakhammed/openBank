use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::shared::types::{AccountId, Amount, Currency, UserId};

/// Balance model for database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Balance {
    pub id: Uuid,
    pub account_id: AccountId,
    pub available_balance: Amount,
    pub ledger_balance: Amount,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Balance history entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BalanceHistory {
    pub id: Uuid,
    pub account_id: AccountId,
    pub balance_before: Amount,
    pub balance_after: Amount,
    pub amount_changed: Amount,
    pub transaction_id: Option<Uuid>,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

/// Balance response
#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub account_id: AccountId,
    pub available_balance: Amount,
    pub ledger_balance: Amount,
    pub currency: Currency,
    pub last_updated: DateTime<Utc>,
}

impl From<Balance> for BalanceResponse {
    fn from(balance: Balance) -> Self {
        Self {
            account_id: balance.account_id,
            available_balance: balance.available_balance,
            ledger_balance: balance.ledger_balance,
            currency: balance.currency,
            last_updated: balance.updated_at,
        }
    }
}

/// User profile model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: UserId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User account model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserAccount {
    pub id: AccountId,
    pub user_id: UserId,
    pub account_number: String,
    pub account_name: String,
    pub account_type: String,
    pub currency: Currency,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User profile response
#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: UserId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl From<UserProfile> for UserProfileResponse {
    fn from(profile: UserProfile) -> Self {
        Self {
            id: profile.id,
            email: profile.email,
            first_name: profile.first_name,
            last_name: profile.last_name,
            phone: profile.phone,
            is_verified: profile.is_verified,
            created_at: profile.created_at,
        }
    }
}

/// User account response
#[derive(Debug, Serialize)]
pub struct UserAccountResponse {
    pub id: AccountId,
    pub account_number: String,
    pub account_name: String,
    pub account_type: String,
    pub currency: Currency,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<UserAccount> for UserAccountResponse {
    fn from(account: UserAccount) -> Self {
        Self {
            id: account.id,
            account_number: account.account_number,
            account_name: account.account_name,
            account_type: account.account_type,
            currency: account.currency,
            is_active: account.is_active,
            created_at: account.created_at,
        }
    }
}
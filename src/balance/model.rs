use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::shared::types::{AccountId, Amount, Currency};

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
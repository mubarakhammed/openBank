use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use validator::Validate;
use crate::shared::types::{AccountId, Amount, Currency, TransactionId};

/// Transaction status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

/// Transaction type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Payment,
    Refund,
}

/// Transaction model for database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: TransactionId,
    pub from_account_id: Option<AccountId>,
    pub to_account_id: Option<AccountId>,
    pub amount: Amount,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub reference: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create transaction request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTransactionRequest {
    pub from_account_id: Option<AccountId>,
    pub to_account_id: Option<AccountId>,
    #[validate(range(min = 1))]
    pub amount: Amount,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Transfer request
#[derive(Debug, Deserialize, Validate)]
pub struct TransferRequest {
    pub from_account_id: AccountId,
    pub to_account_id: AccountId,
    #[validate(range(min = 1))]
    pub amount: Amount,
    pub currency: Currency,
    pub description: Option<String>,
}

/// Transaction response
#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: TransactionId,
    pub from_account_id: Option<AccountId>,
    pub to_account_id: Option<AccountId>,
    pub amount: Amount,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub reference: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Transaction> for TransactionResponse {
    fn from(transaction: Transaction) -> Self {
        Self {
            id: transaction.id,
            from_account_id: transaction.from_account_id,
            to_account_id: transaction.to_account_id,
            amount: transaction.amount,
            currency: transaction.currency,
            transaction_type: transaction.transaction_type,
            status: transaction.status,
            reference: transaction.reference,
            description: transaction.description,
            created_at: transaction.created_at,
        }
    }
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use crate::shared::types::{AccountId, Amount, Currency};

/// Payment status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
}

/// Payment method enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_method", rename_all = "lowercase")]
pub enum PaymentMethod {
    BankTransfer,
    Card,
    Wallet,
    Crypto,
}

/// Payment model for database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub from_account_id: AccountId,
    pub to_account_id: Option<AccountId>,
    pub amount: Amount,
    pub currency: Currency,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub reference: String,
    pub description: Option<String>,
    pub recipient_info: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub external_reference: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create payment request
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePaymentRequest {
    pub to_account_id: Option<AccountId>,
    #[validate(range(min = 1))]
    pub amount: Amount,
    pub currency: Currency,
    pub payment_method: PaymentMethod,
    pub description: Option<String>,
    pub recipient_info: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Payment response
#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub id: Uuid,
    pub from_account_id: AccountId,
    pub to_account_id: Option<AccountId>,
    pub amount: Amount,
    pub currency: Currency,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub reference: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Payment> for PaymentResponse {
    fn from(payment: Payment) -> Self {
        Self {
            id: payment.id,
            from_account_id: payment.from_account_id,
            to_account_id: payment.to_account_id,
            amount: payment.amount,
            currency: payment.currency,
            payment_method: payment.payment_method,
            status: payment.status,
            reference: payment.reference,
            description: payment.description,
            created_at: payment.created_at,
        }
    }
}
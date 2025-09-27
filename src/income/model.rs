use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use crate::shared::types::{UserId, Amount, Currency};

/// Income verification status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "income_verification_status", rename_all = "lowercase")]
pub enum IncomeVerificationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Expired,
}

/// Income verification model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IncomeVerification {
    pub id: Uuid,
    pub user_id: UserId,
    pub verification_type: String,
    pub status: IncomeVerificationStatus,
    pub employer_name: Option<String>,
    pub job_title: Option<String>,
    pub annual_income: Option<Amount>,
    pub currency: Currency,
    pub verification_data: Option<serde_json::Value>,
    pub provider: Option<String>,
    pub provider_reference: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Income verification request
#[derive(Debug, Deserialize, Validate)]
pub struct IncomeVerificationRequest {
    pub verification_type: String,
    pub employer_name: String,
    pub job_title: String,
    pub expected_annual_income: Amount,
    pub currency: Currency,
    pub additional_data: Option<serde_json::Value>,
}

/// Income verification response
#[derive(Debug, Serialize)]
pub struct IncomeVerificationResponse {
    pub id: Uuid,
    pub status: IncomeVerificationStatus,
    pub verification_type: String,
    pub employer_name: Option<String>,
    pub job_title: Option<String>,
    pub annual_income: Option<Amount>,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<IncomeVerification> for IncomeVerificationResponse {
    fn from(verification: IncomeVerification) -> Self {
        Self {
            id: verification.id,
            status: verification.status,
            verification_type: verification.verification_type,
            employer_name: verification.employer_name,
            job_title: verification.job_title,
            annual_income: verification.annual_income,
            currency: verification.currency,
            created_at: verification.created_at,
            completed_at: verification.completed_at,
        }
    }
}
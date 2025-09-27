use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use crate::shared::types::UserId;

/// Identity verification status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verification_status", rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Expired,
}

/// Identity verification model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IdentityVerification {
    pub id: Uuid,
    pub user_id: UserId,
    pub verification_type: String,
    pub status: VerificationStatus,
    pub document_type: Option<String>,
    pub document_number: Option<String>,
    pub verification_data: Option<serde_json::Value>,
    pub provider: Option<String>,
    pub provider_reference: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Identity verification request
#[derive(Debug, Deserialize, Validate)]
pub struct VerificationRequest {
    pub verification_type: String,
    pub document_type: String,
    pub document_number: String,
    pub additional_data: Option<serde_json::Value>,
}

/// Verification response
#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub id: Uuid,
    pub status: VerificationStatus,
    pub verification_type: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl From<IdentityVerification> for VerificationResponse {
    fn from(verification: IdentityVerification) -> Self {
        Self {
            id: verification.id,
            status: verification.status,
            verification_type: verification.verification_type,
            created_at: verification.created_at,
            completed_at: verification.completed_at,
        }
    }
}
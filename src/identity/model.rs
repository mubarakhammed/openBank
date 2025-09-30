use crate::shared::types::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Core identity verification models and types

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BiometricType {
    Face,
    Fingerprint,
    Iris,
    Voice,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Expired,
    Verified,
    Flagged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FraudRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LivenessType {
    Passive,
    Active,
    Enhanced,
}

/// Face embedding stored in the database
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FaceEmbedding {
    pub id: Uuid,
    pub user_id: UserId,
    pub embedding: Vec<f32>, // Face embedding vector
    pub model_version: String,
    pub enrollment_date: DateTime<Utc>,
    pub quality_score: f32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Identity verification record (updated)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct IdentityVerification {
    pub id: Uuid,
    pub user_id: UserId,
    pub verification_type: String, // face, document, etc.
    pub biometric_type: Option<String>,
    pub status: VerificationStatus,
    pub confidence_score: Option<f32>,
    pub liveness_score: Option<f32>,
    pub fraud_risk_score: Option<f32>,
    pub document_type: Option<String>,
    pub document_number: Option<String>,
    pub verification_data: Option<serde_json::Value>,
    pub provider: Option<String>,
    pub provider_reference: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Fraud detection record
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FraudAlert {
    pub id: Uuid,
    pub user_id: UserId,
    pub duplicate_user_id: Option<UserId>,
    pub similarity_score: f32,
    pub alert_type: String,
    pub status: String, // active, resolved, false_positive
    pub investigated_by: Option<UserId>,
    pub investigation_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request models for biometric API endpoints

#[derive(Debug, Deserialize, Validate)]
pub struct FaceMatchRequest {
    #[validate(length(min = 1))]
    pub selfie_image: String, // base64 encoded
    #[validate(length(min = 1))]
    pub id_image: String, // base64 encoded
    pub user_id: Option<UserId>,
    pub include_quality_check: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LivenessRequest {
    #[validate(length(min = 1))]
    pub images: Vec<String>, // base64 encoded images/video frames
    pub liveness_type: LivenessType,
    pub user_id: Option<UserId>,
    pub challenge_response: Option<String>, // for active liveness
}

#[derive(Debug, Deserialize, Validate)]
pub struct EnrollmentRequest {
    pub user_id: UserId,
    #[validate(length(min = 1))]
    pub selfie_image: String, // base64 encoded
    pub perform_liveness_check: Option<bool>,
    pub replace_existing: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct BiometricVerificationRequest {
    pub user_id: UserId,
    #[validate(length(min = 1))]
    pub selfie_image: String, // base64 encoded
    pub perform_liveness_check: Option<bool>,
    #[validate(range(min = 0.5, max = 1.0))]
    pub similarity_threshold: Option<f32>, // default 0.85
}

#[derive(Debug, Deserialize, Validate)]
pub struct FraudCheckRequest {
    pub user_id: UserId,
    #[validate(length(min = 1))]
    pub selfie_image: String, // base64 encoded
    #[validate(range(min = 0.5, max = 1.0))]
    pub fraud_threshold: Option<f32>, // default 0.9
}

/// Legacy document verification request (kept for compatibility)
#[derive(Debug, Deserialize, Validate)]
pub struct VerificationRequest {
    pub verification_type: String,
    pub document_type: String,
    pub document_number: String,
    pub additional_data: Option<serde_json::Value>,
}

/// Response models

#[derive(Debug, Serialize)]
pub struct FaceMatchResponse {
    pub match_score: f32,
    pub is_match: bool,
    pub confidence: f32,
    pub quality_scores: QualityScores,
    pub processing_time_ms: u64,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct LivenessResponse {
    pub is_live: bool,
    pub confidence_score: f32,
    pub liveness_type: LivenessType,
    pub spoof_probability: f32,
    pub quality_scores: QualityScores,
    pub processing_time_ms: u64,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct EnrollmentResponse {
    pub enrollment_id: Uuid,
    pub user_id: UserId,
    pub status: String, // success, failed, duplicate_detected
    pub quality_score: f32,
    pub fraud_risk_score: f32,
    pub processing_time_ms: u64,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct BiometricVerificationResponse {
    pub verified: bool,
    pub similarity_score: f32,
    pub confidence: f32,
    pub liveness_passed: Option<bool>,
    pub fraud_risk_score: Option<f32>,
    pub processing_time_ms: u64,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct FraudCheckResponse {
    pub fraud_risk: String, // low, medium, high, critical
    pub fraud_score: f32,
    pub duplicate_users: Vec<DuplicateUserInfo>,
    pub recommendations: Vec<String>,
    pub processing_time_ms: u64,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct DuplicateUserInfo {
    pub user_id: UserId,
    pub similarity_score: f32,
    pub enrollment_date: DateTime<Utc>,
    pub last_verification: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct QualityScores {
    pub brightness: f32,
    pub sharpness: f32,
    pub face_size: f32,
    pub face_angle: f32,
    pub eye_distance: f32,
    pub overall_quality: f32,
}

/// Legacy verification response (kept for compatibility)
#[derive(Debug, Serialize)]
pub struct VerificationResponse {
    pub id: Uuid,
    pub status: VerificationStatus,
    pub verification_type: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// ML Model configuration
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub face_detection_model_path: String,
    pub face_recognition_model_path: String,
    pub liveness_model_path: String,
    pub anti_spoof_model_path: String,
    pub model_version: String,
    pub input_size: (u32, u32), // width, height
    pub embedding_size: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            face_detection_model_path: "models/face_detection.onnx".to_string(),
            face_recognition_model_path: "models/arcface.onnx".to_string(),
            liveness_model_path: "models/liveness.onnx".to_string(),
            anti_spoof_model_path: "models/anti_spoof.onnx".to_string(),
            model_version: "v1.0.0".to_string(),
            input_size: (112, 112),
            embedding_size: 512,
        }
    }
}

/// Error types for identity verification
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    #[error("Invalid image format: {0}")]
    InvalidImageFormat(String),

    #[error("Face not detected in image")]
    FaceNotDetected,

    #[error("Multiple faces detected")]
    MultipleFacesDetected,

    #[error("Image quality too low: {0}")]
    LowImageQuality(String),

    #[error("Liveness check failed: {0}")]
    LivenessCheckFailed(String),

    #[error("Model inference error: {0}")]
    ModelInferenceError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Enrollment not found for user: {0}")]
    EnrollmentNotFound(UserId),

    #[error("Duplicate enrollment detected")]
    DuplicateEnrollment,

    #[error("Fraud detected: {0}")]
    FraudDetected(String),

    #[error("Invalid threshold value: {0}")]
    InvalidThreshold(f32),

    #[error("Processing timeout")]
    ProcessingTimeout,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("No face detected in image")]
    NoFaceDetected,

    #[error("Image quality too low: {0}")]
    QualityTooLow(String),

    #[error("Spoofing detected")]
    SpoofingDetected,

    #[error("Face does not match")]
    FaceNotMatched,

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Fraud detection error: {0}")]
    FraudDetectionError(String),

    #[error("Document parsing error: {0}")]
    DocumentParsingError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

/// Type alias for Result with IdentityError
pub type IdentityResult<T> = Result<T, IdentityError>;

/// Constants for identity verification
pub mod constants {
    pub const DEFAULT_SIMILARITY_THRESHOLD: f32 = 0.85;
    pub const DEFAULT_FRAUD_THRESHOLD: f32 = 0.9;
    pub const MIN_FACE_SIZE: f32 = 80.0; // minimum face size in pixels
    pub const MAX_FACE_ANGLE: f32 = 30.0; // maximum face angle in degrees
    pub const MIN_QUALITY_SCORE: f32 = 0.6;
    pub const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
    pub const SUPPORTED_IMAGE_FORMATS: &[&str] = &["jpeg", "jpg", "png", "webp"];
    pub const MAX_PROCESSING_TIME_MS: u64 = 5000; // 5 seconds
    pub const EMBEDDING_CACHE_TTL_HOURS: i64 = 24;
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

// Export constants at module level for easier access
pub use constants::*;

// Additional constants needed by services
pub const SIMILARITY_THRESHOLD: f32 = constants::DEFAULT_SIMILARITY_THRESHOLD;
pub const MIN_QUALITY_SCORE: f32 = constants::MIN_QUALITY_SCORE;
pub const MAX_PROCESSING_TIME_MS: u64 = constants::MAX_PROCESSING_TIME_MS;

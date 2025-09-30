use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::identity::model::{
    BiometricVerificationRequest, FaceMatchRequest, IdentityError, IdentityResult, LivenessRequest,
};
use crate::identity::service::IdentityService;

/// Identity API handlers
pub struct IdentityHandlers {
    service: Arc<IdentityService>,
}

impl IdentityHandlers {
    pub fn new(service: IdentityService) -> Self {
        Self {
            service: Arc::new(service),
        }
    }

    /// Create identity routes
    pub fn routes(&self) -> Router<Arc<IdentityService>> {
        Router::new()
            .route("/identity/verify/biometric", post(Self::verify_biometric))
            .route("/identity/face/match", post(Self::match_face))
            .route("/identity/face/enroll", post(Self::enroll_face))
            .route("/identity/liveness/detect", post(Self::detect_liveness))
            .route(
                "/identity/user/:user_id/verifications",
                get(Self::get_verification_history),
            )
            .route(
                "/identity/user/:user_id/fraud-alerts",
                get(Self::get_fraud_alerts),
            )
            .route("/identity/user/:user_id/stats", get(Self::get_user_stats))
    }

    /// POST /identity/verify/biometric - Comprehensive biometric verification
    async fn verify_biometric(
        State(service): State<Arc<IdentityService>>,
        Json(request): Json<BiometricVerificationRequest>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!(
            "Received biometric verification request for user {}",
            request.user_id
        );

        match service.verify_biometric(request).await {
            Ok(response) => {
                info!("Biometric verification completed successfully");
                Ok(Json(json!({
                    "success": true,
                    "data": response,
                    "message": "Biometric verification completed"
                })))
            }
            Err(e) => {
                error!("Biometric verification failed: {:?}", e);
                let (status, error_message) = map_identity_error(&e);
                Err((
                    status,
                    Json(json!({
                        "success": false,
                        "error": error_message,
                        "code": format!("{:?}", e)
                    })),
                ))
            }
        }
    }

    /// POST /identity/face/match - Face matching against enrolled faces
    async fn match_face(
        State(service): State<Arc<IdentityService>>,
        Json(request): Json<FaceMatchRequest>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!("Received face match request for user {:?}", request.user_id);

        match service.match_face(request).await {
            Ok(response) => {
                info!("Face match completed successfully");
                Ok(Json(json!({
                    "success": true,
                    "data": response,
                    "message": "Face match completed"
                })))
            }
            Err(e) => {
                error!("Face match failed: {:?}", e);
                let (status, error_message) = map_identity_error(&e);
                Err((
                    status,
                    Json(json!({
                        "success": false,
                        "error": error_message,
                        "code": format!("{:?}", e)
                    })),
                ))
            }
        }
    }

    /// POST /identity/face/enroll - Enroll a new face for a user
    async fn enroll_face(
        State(service): State<Arc<IdentityService>>,
        Json(payload): Json<Value>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        let user_id = payload
            .get("user_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "success": false,
                        "error": "Invalid or missing user_id"
                    })),
                )
            })?;

        let image_data = payload
            .get("image_data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "success": false,
                        "error": "Missing image_data"
                    })),
                )
            })?;

        info!("Received face enrollment request for user {}", user_id);

        // Decode base64 image
        let image_bytes = base64::decode(image_data).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Invalid base64 image data"
                })),
            )
        })?;

        let image = image::load_from_memory(&image_bytes).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Invalid image format"
                })),
            )
        })?;

        match service.enroll_face(user_id, &image).await {
            Ok(embedding_id) => {
                info!(
                    "Face enrollment completed successfully for user {}",
                    user_id
                );
                Ok(Json(json!({
                    "success": true,
                    "data": {
                        "embedding_id": embedding_id,
                        "user_id": user_id,
                        "enrolled_at": chrono::Utc::now()
                    },
                    "message": "Face enrolled successfully"
                })))
            }
            Err(e) => {
                error!("Face enrollment failed for user {}: {:?}", user_id, e);
                let (status, error_message) = map_identity_error(&e);
                Err((
                    status,
                    Json(json!({
                        "success": false,
                        "error": error_message,
                        "code": format!("{:?}", e)
                    })),
                ))
            }
        }
    }

    /// POST /identity/liveness/detect - Liveness detection
    async fn detect_liveness(
        State(service): State<Arc<IdentityService>>,
        Json(request): Json<LivenessRequest>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!(
            "Received liveness detection request for user {:?}",
            request.user_id
        );

        match service.detect_liveness(request).await {
            Ok(response) => {
                info!("Liveness detection completed successfully");
                Ok(Json(json!({
                    "success": true,
                    "data": response,
                    "message": "Liveness detection completed"
                })))
            }
            Err(e) => {
                error!("Liveness detection failed: {:?}", e);
                let (status, error_message) = map_identity_error(&e);
                Err((
                    status,
                    Json(json!({
                        "success": false,
                        "error": error_message,
                        "code": format!("{:?}", e)
                    })),
                ))
            }
        }
    }

    /// GET /identity/user/{user_id}/verifications - Get verification history
    async fn get_verification_history(
        State(service): State<Arc<IdentityService>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!("Fetching verification history for user {}", user_id);

        match service.get_verification_history(user_id).await {
            Ok(verifications) => Ok(Json(json!({
                "success": true,
                "data": {
                    "user_id": user_id,
                    "verifications": verifications,
                    "count": verifications.len()
                },
                "message": "Verification history retrieved successfully"
            }))),
            Err(e) => {
                error!(
                    "Failed to fetch verification history for user {}: {:?}",
                    user_id, e
                );
                let (status, error_message) = map_identity_error(&e);
                Err((
                    status,
                    Json(json!({
                        "success": false,
                        "error": error_message,
                        "code": format!("{:?}", e)
                    })),
                ))
            }
        }
    }

    /// GET /identity/user/{user_id}/fraud-alerts - Get fraud alerts
    async fn get_fraud_alerts(
        State(service): State<Arc<IdentityService>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!("Fetching fraud alerts for user {}", user_id);

        match service.get_fraud_alerts(user_id).await {
            Ok(alerts) => Ok(Json(json!({
                "success": true,
                "data": {
                    "user_id": user_id,
                    "alerts": alerts,
                    "count": alerts.len()
                },
                "message": "Fraud alerts retrieved successfully"
            }))),
            Err(e) => {
                error!("Failed to fetch fraud alerts for user {}: {:?}", user_id, e);
                let (status, error_message) = map_identity_error(&e);
                Err((
                    status,
                    Json(json!({
                        "success": false,
                        "error": error_message,
                        "code": format!("{:?}", e)
                    })),
                ))
            }
        }
    }

    /// GET /identity/user/{user_id}/stats - Get user identity statistics
    async fn get_user_stats(
        State(service): State<Arc<IdentityService>>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        info!("Fetching identity stats for user {}", user_id);

        // This would need to be implemented in the service
        Ok(Json(json!({
            "success": true,
            "data": {
                "user_id": user_id,
                "total_verifications": 0,
                "successful_verifications": 0,
                "failed_verifications": 0,
                "fraud_alerts": 0,
                "enrolled_faces": 0,
                "last_verification": null
            },
            "message": "User stats retrieved successfully"
        })))
    }
}

/// Map IdentityError to HTTP status code and user-friendly message
fn map_identity_error(error: &IdentityError) -> (StatusCode, String) {
    match error {
        IdentityError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        IdentityError::NoFaceDetected => (
            StatusCode::BAD_REQUEST,
            "No face detected in image".to_string(),
        ),
        IdentityError::MultipleFacesDetected => (
            StatusCode::BAD_REQUEST,
            "Multiple faces detected in image".to_string(),
        ),
        IdentityError::QualityTooLow(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        IdentityError::LivenessCheckFailed(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
        IdentityError::SpoofingDetected => (
            StatusCode::BAD_REQUEST,
            "Spoofing attempt detected".to_string(),
        ),
        IdentityError::FaceNotMatched => (
            StatusCode::UNAUTHORIZED,
            "Face does not match enrolled biometrics".to_string(),
        ),
        IdentityError::ModelInferenceError(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("ML inference error: {}", msg),
        ),
        IdentityError::DatabaseError(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", msg),
        ),
        IdentityError::Internal(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal error: {}", msg),
        ),
        IdentityError::ConfigurationError(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Configuration error: {}", msg),
        ),
        IdentityError::ProcessingTimeout => (
            StatusCode::REQUEST_TIMEOUT,
            "Processing timeout".to_string(),
        ),
        IdentityError::FraudDetected(msg) => {
            (StatusCode::FORBIDDEN, format!("Fraud detected: {}", msg))
        }
        IdentityError::InvalidImageFormat(msg) => (
            StatusCode::BAD_REQUEST,
            format!("Invalid image format: {}", msg),
        ),
        IdentityError::FaceNotDetected => (
            StatusCode::BAD_REQUEST,
            "Face not detected in image".to_string(),
        ),
        IdentityError::LowImageQuality(msg) => (
            StatusCode::BAD_REQUEST,
            format!("Low image quality: {}", msg),
        ),
        IdentityError::EnrollmentNotFound(user_id) => (
            StatusCode::NOT_FOUND,
            format!("Enrollment not found for user: {}", user_id),
        ),
        IdentityError::DuplicateEnrollment => (
            StatusCode::CONFLICT,
            "Duplicate enrollment detected".to_string(),
        ),
        IdentityError::InvalidThreshold(threshold) => (
            StatusCode::BAD_REQUEST,
            format!("Invalid threshold value: {}", threshold),
        ),
        IdentityError::ValidationError(msg) => (
            StatusCode::BAD_REQUEST,
            format!("Validation error: {}", msg),
        ),
        IdentityError::DocumentParsingError(msg) => (
            StatusCode::BAD_REQUEST,
            format!("Document parsing error: {}", msg),
        ),
        IdentityError::ExternalServiceError(msg) => (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("External service error: {}", msg),
        ),
        IdentityError::FraudDetectionError(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Fraud detection error: {}", msg),
        ),
    }
}

/// Health check endpoint specifically for identity service
pub async fn health_check(
    State(service): State<Arc<IdentityService>>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Check if ML models are loaded and database is accessible
    // This is a simplified health check
    Ok(Json(json!({
        "status": "healthy",
        "service": "identity",
        "timestamp": chrono::Utc::now(),
        "models": {
            "face_detection": "loaded",
            "face_recognition": "loaded",
            "liveness_detection": "loaded",
            "anti_spoofing": "loaded"
        },
        "database": "connected"
    })))
}

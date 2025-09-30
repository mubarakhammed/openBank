use anyhow::Result;
use image::DynamicImage;
use std::sync::Arc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::identity::ml_service::{
    AntiSpoofResult, FaceDetection, LivenessResult, MLInferenceService,
};
use crate::identity::model::{
    BiometricVerificationRequest, BiometricVerificationResponse, FaceEmbedding, FaceMatchRequest,
    FaceMatchResponse, FraudAlert, FraudRiskLevel, IdentityError, IdentityResult,
    IdentityVerification, LivenessRequest, LivenessResponse, ModelConfig, QualityScores,
    VerificationStatus, MAX_PROCESSING_TIME_MS, MIN_QUALITY_SCORE, SIMILARITY_THRESHOLD,
};
use crate::identity::repository_new::IdentityRepository;

/// Main identity service that orchestrates all identity verification operations
#[derive(Clone)]
pub struct IdentityService {
    ml_service: Arc<MLInferenceService>,
    repository: Arc<IdentityRepository>,
    config: ModelConfig,
}

impl IdentityService {
    /// Create a new identity service
    pub async fn new(repository: IdentityRepository, config: ModelConfig) -> Result<Self> {
        let ml_service = Arc::new(MLInferenceService::new(config.clone()).await?);

        Ok(Self {
            ml_service,
            repository: Arc::new(repository),
            config,
        })
    }

    /// Initialize the service and database
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing identity service...");
        self.repository.initialize().await?;
        info!("Identity service initialized successfully");
        Ok(())
    }

    /// Perform comprehensive biometric verification
    pub async fn verify_biometric(
        &self,
        request: BiometricVerificationRequest,
    ) -> IdentityResult<BiometricVerificationResponse> {
        let start_time = std::time::Instant::now();
        info!(
            "Starting biometric verification for user {}",
            request.user_id
        );

        // Decode base64 image
        let image_data = base64::decode(&request.selfie_image).map_err(|e| {
            IdentityError::InvalidImageFormat(format!("Invalid base64 image: {}", e))
        })?;

        let image = image::load_from_memory(&image_data).map_err(|e| {
            IdentityError::InvalidImageFormat(format!("Invalid image format: {}", e))
        })?;

        // Perform all verification steps
        let verification_result = self
            .perform_comprehensive_verification(request.user_id, &image, "biometric")
            .await?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Check if processing took too long
        if processing_time > MAX_PROCESSING_TIME_MS {
            warn!(
                "Verification took too long: {}ms for user {}",
                processing_time, request.user_id
            );
        }

        info!(
            "Biometric verification completed for user {} in {}ms",
            request.user_id, processing_time
        );

        let is_verified = verification_result.status == VerificationStatus::Completed;
        Ok(BiometricVerificationResponse {
            verified: is_verified,
            similarity_score: verification_result.face_match_score.unwrap_or(0.0),
            confidence: verification_result.confidence_score,
            liveness_passed: verification_result.liveness_score.map(|score| score > 0.5),
            fraud_risk_score: Some(if verification_result.fraud_indicators.is_empty() {
                0.1
            } else {
                0.8
            }),
            processing_time_ms: processing_time as u64,
            request_id: verification_result.verification_id,
        })
    }

    /// Perform face matching against enrolled faces
    pub async fn match_face(&self, request: FaceMatchRequest) -> IdentityResult<FaceMatchResponse> {
        info!("Starting face match for user {:?}", request.user_id);

        // Decode image
        let image_data = base64::decode(&request.selfie_image).map_err(|e| {
            IdentityError::InvalidImageFormat(format!("Invalid base64 image: {}", e))
        })?;

        let image = image::load_from_memory(&image_data).map_err(|e| {
            IdentityError::InvalidImageFormat(format!("Invalid image format: {}", e))
        })?;

        // Extract face embedding
        let embedding = self.ml_service.extract_face_embedding(&image).await?;

        // Find similar embeddings in database
        let similar_embeddings = self
            .repository
            .find_similar_embeddings(
                &embedding,
                SIMILARITY_THRESHOLD,
                10, // Limit to top 10 matches
            )
            .await?;

        // Calculate match score
        let (is_match, confidence_score, matched_user_id) = if similar_embeddings.is_empty() {
            (false, 0.0, None)
        } else {
            let best_match = &similar_embeddings[0];
            let similarity = self
                .ml_service
                .compare_embeddings(&embedding, &best_match.embedding)?;
            let is_match = similarity >= SIMILARITY_THRESHOLD;
            (
                is_match,
                similarity,
                if is_match {
                    Some(best_match.user_id)
                } else {
                    None
                },
            )
        };

        // Check for potential fraud if multiple similar faces found
        if similar_embeddings.len() > 1 {
            self.check_for_duplicate_enrollment(&embedding, &similar_embeddings)
                .await?;
        }

        let quality_scores = self.ml_service.calculate_quality_scores(&image)?;
        Ok(FaceMatchResponse {
            match_score: confidence_score,
            is_match,
            confidence: confidence_score,
            quality_scores,
            processing_time_ms: 0, // TODO: Track actual processing time
            request_id: Uuid::new_v4(),
        })
    }

    /// Perform liveness detection
    pub async fn detect_liveness(
        &self,
        request: LivenessRequest,
    ) -> IdentityResult<LivenessResponse> {
        info!("Starting liveness detection for user {:?}", request.user_id);

        // Decode first image
        let image_data = base64::decode(&request.images[0]).map_err(|e| {
            IdentityError::InvalidImageFormat(format!("Invalid base64 image: {}", e))
        })?;

        let image = image::load_from_memory(&image_data).map_err(|e| {
            IdentityError::InvalidImageFormat(format!("Invalid image format: {}", e))
        })?;

        // Perform liveness detection
        let liveness_result = self.ml_service.detect_liveness(&image).await?;

        // Perform anti-spoofing detection
        let antispoof_result = self.ml_service.detect_anti_spoof(&image).await?;

        // Combine results
        let is_live = liveness_result.is_live && antispoof_result.is_real;
        let confidence_score = (liveness_result.confidence + antispoof_result.confidence) / 2.0;

        let quality_scores = self.ml_service.calculate_quality_scores(&image)?;
        Ok(LivenessResponse {
            is_live,
            confidence_score,
            liveness_type: request.liveness_type,
            spoof_probability: liveness_result.spoof_probability,
            quality_scores,
            processing_time_ms: 0, // TODO: Track actual processing time
            request_id: Uuid::new_v4(),
        })
    }

    /// Enroll a new face for a user
    pub async fn enroll_face(&self, user_id: Uuid, image: &DynamicImage) -> IdentityResult<Uuid> {
        info!("Enrolling face for user {}", user_id);

        // Check image quality
        let quality_scores = self.ml_service.calculate_quality_scores(image)?;
        if quality_scores.overall_quality < MIN_QUALITY_SCORE {
            return Err(IdentityError::QualityTooLow(format!(
                "Image quality too low: {:.2} (minimum: {:.2})",
                quality_scores.overall_quality, MIN_QUALITY_SCORE
            )));
        }

        // Detect faces
        let face_detections = self.ml_service.detect_faces(image).await?;
        if face_detections.is_empty() {
            return Err(IdentityError::NoFaceDetected);
        }
        if face_detections.len() > 1 {
            return Err(IdentityError::MultipleFacesDetected);
        }

        // Extract face embedding
        let embedding = self.ml_service.extract_face_embedding(image).await?;

        // Check for existing similar faces (duplicate enrollment prevention)
        let existing_embeddings = self
            .repository
            .find_similar_embeddings(
                &embedding,
                SIMILARITY_THRESHOLD * 1.1, // Slightly higher threshold for enrollment
                5,
            )
            .await?;

        if !existing_embeddings.is_empty() {
            // Check if any existing embedding belongs to a different user
            for existing in &existing_embeddings {
                if existing.user_id != user_id {
                    warn!(
                        "Potential duplicate enrollment detected for user {}",
                        user_id
                    );
                    self.create_fraud_alert(
                        user_id,
                        Some(existing.user_id),
                        "duplicate_enrollment".to_string(),
                        self.ml_service
                            .compare_embeddings(&embedding, &existing.embedding)?,
                    )
                    .await?;
                }
            }
        }

        // Create face embedding record
        let face_embedding = FaceEmbedding {
            id: Uuid::new_v4(),
            user_id,
            embedding,
            model_version: self.config.model_version.clone(),
            quality_score: quality_scores.overall_quality,
            enrollment_date: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            is_active: true,
        };

        // Store in database
        let embedding_id = self
            .repository
            .store_face_embedding(&face_embedding)
            .await?;

        // TODO: Implement deactivate_old_embeddings if needed
        // Keep only the 3 most recent embeddings per user

        info!(
            "Face enrolled successfully for user {}: {}",
            user_id, embedding_id
        );
        Ok(embedding_id)
    }

    /// Get verification history for a user
    pub async fn get_verification_history(
        &self,
        user_id: Uuid,
    ) -> IdentityResult<Vec<IdentityVerification>> {
        // This would need to be implemented in the repository
        Ok(Vec::new())
    }

    /// Get fraud alerts for a user
    pub async fn get_fraud_alerts(&self, user_id: Uuid) -> IdentityResult<Vec<FraudAlert>> {
        self.repository.get_user_fraud_alerts(user_id).await
    }

    // Private helper methods

    /// Perform comprehensive verification including all checks
    async fn perform_comprehensive_verification(
        &self,
        user_id: Uuid,
        image: &DynamicImage,
        verification_type: &str,
    ) -> IdentityResult<ComprehensiveVerificationResult> {
        // 1. Quality Assessment
        let quality_scores = self.ml_service.calculate_quality_scores(image)?;
        if quality_scores.overall_quality < MIN_QUALITY_SCORE {
            return Ok(ComprehensiveVerificationResult {
                verification_id: Uuid::new_v4(),
                status: VerificationStatus::Failed,
                confidence_score: 0.0,
                face_match_score: None,
                liveness_score: None,
                quality_scores: Some(quality_scores),
                fraud_indicators: vec!["low_quality".to_string()],
            });
        }

        // 2. Face Detection
        let face_detections = self.ml_service.detect_faces(image).await?;
        if face_detections.is_empty() {
            return Ok(ComprehensiveVerificationResult {
                verification_id: Uuid::new_v4(),
                status: VerificationStatus::Failed,
                confidence_score: 0.0,
                face_match_score: None,
                liveness_score: None,
                quality_scores: Some(quality_scores),
                fraud_indicators: vec!["no_face_detected".to_string()],
            });
        }

        // 3. Liveness Detection
        let liveness_result = self.ml_service.detect_liveness(image).await?;
        if !liveness_result.is_live {
            return Ok(ComprehensiveVerificationResult {
                verification_id: Uuid::new_v4(),
                status: VerificationStatus::Failed,
                confidence_score: liveness_result.confidence,
                face_match_score: None,
                liveness_score: Some(liveness_result.confidence),
                quality_scores: Some(quality_scores),
                fraud_indicators: vec!["liveness_failed".to_string()],
            });
        }

        // 4. Anti-Spoofing
        let antispoof_result = self.ml_service.detect_anti_spoof(image).await?;
        if !antispoof_result.is_real {
            return Ok(ComprehensiveVerificationResult {
                verification_id: Uuid::new_v4(),
                status: VerificationStatus::Failed,
                confidence_score: antispoof_result.confidence,
                face_match_score: None,
                liveness_score: Some(liveness_result.confidence),
                quality_scores: Some(quality_scores),
                fraud_indicators: vec!["spoofing_detected".to_string()],
            });
        }

        // 5. Face Matching
        let embedding = self.ml_service.extract_face_embedding(image).await?;
        let user_embeddings = self.repository.get_user_embeddings(user_id).await?;

        let (face_match_score, is_match) = if user_embeddings.is_empty() {
            (0.0, false)
        } else {
            let mut best_score = 0.0;
            for user_embedding in &user_embeddings {
                let similarity = self
                    .ml_service
                    .compare_embeddings(&embedding, &user_embedding.embedding)?;
                if similarity > best_score {
                    best_score = similarity;
                }
            }
            (best_score, best_score >= SIMILARITY_THRESHOLD)
        };

        // 6. Final Verification Decision
        let mut fraud_indicators = Vec::new();
        let status = if is_match {
            VerificationStatus::Verified
        } else {
            fraud_indicators.push("face_mismatch".to_string());
            VerificationStatus::Failed
        };

        // Calculate overall confidence
        let confidence_score = (quality_scores.overall_quality * 0.2
            + liveness_result.confidence * 0.3
            + antispoof_result.confidence * 0.2
            + face_match_score * 0.3)
            .min(1.0);

        // Store verification record
        let verification = IdentityVerification {
            id: Uuid::new_v4(),
            user_id,
            verification_type: verification_type.to_string(),
            biometric_type: Some("face".to_string()),
            status: status.clone(),
            confidence_score: Some(confidence_score),
            liveness_score: Some(liveness_result.confidence),
            fraud_risk_score: Some(1.0 - confidence_score),
            document_type: None,
            document_number: None,
            verification_data: None,
            provider: Some("openbank_ml".to_string()),
            provider_reference: None,
            completed_at: Some(chrono::Utc::now()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let verification_id = self.repository.store_verification(&verification).await?;

        Ok(ComprehensiveVerificationResult {
            verification_id,
            status,
            confidence_score,
            face_match_score: Some(face_match_score),
            liveness_score: Some(liveness_result.confidence),
            quality_scores: Some(quality_scores),
            fraud_indicators,
        })
    }

    /// Check for potential duplicate enrollment fraud
    async fn check_for_duplicate_enrollment(
        &self,
        embedding: &[f32],
        similar_embeddings: &[FaceEmbedding],
    ) -> IdentityResult<()> {
        for similar_embedding in similar_embeddings {
            let similarity = self
                .ml_service
                .compare_embeddings(embedding, &similar_embedding.embedding)?;

            if similarity > SIMILARITY_THRESHOLD * 1.2 {
                warn!(
                    "High similarity detected between different users: {:.3}",
                    similarity
                );
                // This would be handled by creating fraud alerts in a real implementation
            }
        }
        Ok(())
    }

    /// Create a fraud alert
    async fn create_fraud_alert(
        &self,
        user_id: Uuid,
        duplicate_user_id: Option<Uuid>,
        alert_type: String,
        similarity_score: f32,
    ) -> IdentityResult<Uuid> {
        use crate::identity::repository_new::FraudStatus;

        let fraud_alert = FraudAlert {
            id: Uuid::new_v4(),
            user_id,
            duplicate_user_id,
            alert_type,
            similarity_score,
            status: "active".to_string(),
            investigated_by: None,
            investigation_notes: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.repository.store_fraud_alert(&fraud_alert).await
    }
}

/// Internal result type for comprehensive verification
#[derive(Debug)]
struct ComprehensiveVerificationResult {
    verification_id: Uuid,
    status: VerificationStatus,
    confidence_score: f32,
    face_match_score: Option<f32>,
    liveness_score: Option<f32>,
    quality_scores: Option<QualityScores>,
    fraud_indicators: Vec<String>,
}

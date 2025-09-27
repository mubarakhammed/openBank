use uuid::Uuid;
use chrono::Utc;
use crate::core::error::{AppError, AppResult};
use crate::shared::{traits::Repository, types::UserId};
use super::model::{
    IdentityVerification, VerificationRequest, VerificationResponse, VerificationStatus
};
use super::repository::IdentityRepository;

pub struct IdentityService {
    repository: IdentityRepository,
}

impl IdentityService {
    pub fn new(repository: IdentityRepository) -> Self {
        Self { repository }
    }

    /// Initiate identity verification
    pub async fn initiate_verification(
        &self,
        user_id: UserId,
        request: VerificationRequest,
    ) -> AppResult<VerificationResponse> {
        // TODO: Implement verification initiation logic
        let now = Utc::now();
        let verification = IdentityVerification {
            id: Uuid::new_v4(),
            user_id,
            verification_type: request.verification_type,
            status: VerificationStatus::Pending,
            document_type: Some(request.document_type),
            document_number: Some(request.document_number),
            verification_data: request.additional_data,
            provider: None,
            provider_reference: None,
            completed_at: None,
            created_at: now,
            updated_at: now,
        };

        let created_verification = self.repository.create(verification).await?;
        Ok(VerificationResponse::from(created_verification))
    }

    /// Get verification status
    pub async fn get_verification_status(&self, verification_id: Uuid) -> AppResult<VerificationResponse> {
        let verification = self.repository.find_by_id(verification_id).await?
            .ok_or_else(|| AppError::NotFound("Verification not found".to_string()))?;

        Ok(VerificationResponse::from(verification))
    }

    /// Get verifications for user
    pub async fn get_user_verifications(&self, user_id: UserId) -> AppResult<Vec<VerificationResponse>> {
        let verifications = self.repository.find_by_user_id(user_id).await?;
        Ok(verifications.into_iter().map(VerificationResponse::from).collect())
    }
}
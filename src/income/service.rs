use uuid::Uuid;
use chrono::Utc;
use crate::core::error::{AppError, AppResult};
use crate::shared::{traits::Repository, types::UserId};
use super::model::{
    IncomeVerification, IncomeVerificationRequest, IncomeVerificationResponse, IncomeVerificationStatus
};
use super::repository::IncomeRepository;

pub struct IncomeService {
    repository: IncomeRepository,
}

impl IncomeService {
    pub fn new(repository: IncomeRepository) -> Self {
        Self { repository }
    }

    /// Initiate income verification
    pub async fn initiate_verification(
        &self,
        user_id: UserId,
        request: IncomeVerificationRequest,
    ) -> AppResult<IncomeVerificationResponse> {
        // TODO: Implement income verification initiation logic
        let now = Utc::now();
        let verification = IncomeVerification {
            id: Uuid::new_v4(),
            user_id,
            verification_type: request.verification_type,
            status: IncomeVerificationStatus::Pending,
            employer_name: Some(request.employer_name),
            job_title: Some(request.job_title),
            annual_income: Some(request.expected_annual_income),
            currency: request.currency,
            verification_data: request.additional_data,
            provider: None,
            provider_reference: None,
            completed_at: None,
            created_at: now,
            updated_at: now,
        };

        let created_verification = self.repository.create(verification).await?;
        Ok(IncomeVerificationResponse::from(created_verification))
    }

    /// Get verification status
    pub async fn get_verification_status(&self, verification_id: Uuid) -> AppResult<IncomeVerificationResponse> {
        let verification = self.repository.find_by_id(verification_id).await?
            .ok_or_else(|| AppError::NotFound("Income verification not found".to_string()))?;

        Ok(IncomeVerificationResponse::from(verification))
    }

    /// Get verifications for user
    pub async fn get_user_verifications(&self, user_id: UserId) -> AppResult<Vec<IncomeVerificationResponse>> {
        let verifications = self.repository.find_by_user_id(user_id).await?;
        Ok(verifications.into_iter().map(IncomeVerificationResponse::from).collect())
    }
}
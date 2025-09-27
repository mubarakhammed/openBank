use uuid::Uuid;
use chrono::Utc;
use crate::core::error::{AppError, AppResult};
use crate::shared::{traits::Repository, types::UserId};
use super::model::{
    VirtualAccount, VirtualAccountResponse, CreateVirtualAccountRequest, VirtualAccountStatus
};
use super::repository::VirtualAccountRepository;

pub struct VirtualAccountService {
    repository: VirtualAccountRepository,
}

impl VirtualAccountService {
    pub fn new(repository: VirtualAccountRepository) -> Self {
        Self { repository }
    }

    /// Create a new virtual account
    pub async fn create_virtual_account(
        &self,
        user_id: UserId,
        request: CreateVirtualAccountRequest,
    ) -> AppResult<VirtualAccountResponse> {
        // TODO: Implement virtual account creation logic
        let account_number = self.repository.generate_account_number().await?;
        let now = Utc::now();
        
        let virtual_account = VirtualAccount {
            id: Uuid::new_v4(),
            user_id,
            parent_account_id: request.parent_account_id,
            account_number,
            account_name: request.account_name,
            currency: request.currency,
            status: VirtualAccountStatus::Active,
            purpose: request.purpose,
            metadata: request.metadata,
            created_at: now,
            updated_at: now,
        };

        let created_account = self.repository.create(virtual_account).await?;
        Ok(VirtualAccountResponse::from(created_account))
    }

    /// Get virtual account by ID
    pub async fn get_virtual_account(&self, account_id: Uuid) -> AppResult<VirtualAccountResponse> {
        let account = self.repository.find_by_id(account_id).await?
            .ok_or_else(|| AppError::NotFound("Virtual account not found".to_string()))?;

        Ok(VirtualAccountResponse::from(account))
    }

    /// Get virtual accounts for user
    pub async fn get_user_virtual_accounts(&self, user_id: UserId) -> AppResult<Vec<VirtualAccountResponse>> {
        let accounts = self.repository.find_by_user_id(user_id).await?;
        Ok(accounts.into_iter().map(VirtualAccountResponse::from).collect())
    }

    /// Deactivate virtual account
    pub async fn deactivate_virtual_account(&self, account_id: Uuid) -> AppResult<()> {
        self.repository.update_status(account_id, VirtualAccountStatus::Inactive).await
    }
}
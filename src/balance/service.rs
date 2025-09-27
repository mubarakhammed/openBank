use super::model::{BalanceHistory, BalanceResponse};
use super::repository::BalanceRepository;
use crate::core::error::{AppError, AppResult};
use crate::shared::{
    traits::Repository,
    types::{AccountId, Amount},
};

pub struct BalanceService {
    repository: BalanceRepository,
}

impl BalanceService {
    pub fn new(repository: BalanceRepository) -> Self {
        Self { repository }
    }

    /// Get current balance for account
    pub async fn get_balance(&self, account_id: AccountId) -> AppResult<BalanceResponse> {
        let balance = self
            .repository
            .find_by_account_id(account_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Balance not found for account".to_string()))?;

        Ok(BalanceResponse::from(balance))
    }

    /// Get balance history for account
    pub async fn get_balance_history(
        &self,
        account_id: AccountId,
        page: u32,
        limit: u32,
    ) -> AppResult<Vec<BalanceHistory>> {
        self.repository
            .get_balance_history(account_id, page, limit)
            .await
    }

    /// Update balance (used by transaction service)
    pub async fn update_balance(
        &self,
        account_id: AccountId,
        amount: Amount,
        description: String,
    ) -> AppResult<BalanceResponse> {
        // TODO: Implement balance update logic
        // 1. Get current balance
        // 2. Calculate new balance
        // 3. Update balance in database
        // 4. Create balance history entry
        // 5. Return updated balance

        // Placeholder implementation
        self.get_balance(account_id).await
    }
}

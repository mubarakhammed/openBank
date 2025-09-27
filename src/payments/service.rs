use uuid::Uuid;
use chrono::Utc;
use crate::core::error::{AppError, AppResult};
use crate::shared::{traits::Repository, types::AccountId};
use super::model::{
    Payment, PaymentResponse, CreatePaymentRequest, PaymentStatus
};
use super::repository::PaymentRepository;

pub struct PaymentService {
    repository: PaymentRepository,
}

impl PaymentService {
    pub fn new(repository: PaymentRepository) -> Self {
        Self { repository }
    }

    /// Create a new payment
    pub async fn create_payment(
        &self,
        from_account_id: AccountId,
        request: CreatePaymentRequest,
    ) -> AppResult<PaymentResponse> {
        // TODO: Implement payment creation logic
        let now = Utc::now();
        let payment = Payment {
            id: Uuid::new_v4(),
            from_account_id,
            to_account_id: request.to_account_id,
            amount: request.amount,
            currency: request.currency,
            payment_method: request.payment_method,
            status: PaymentStatus::Pending,
            reference: format!("PAY_{}", Uuid::new_v4()),
            description: request.description,
            recipient_info: request.recipient_info,
            metadata: request.metadata,
            external_reference: None,
            created_at: now,
            updated_at: now,
        };

        let created_payment = self.repository.create(payment).await?;
        Ok(PaymentResponse::from(created_payment))
    }

    /// Get payment by ID
    pub async fn get_payment(&self, payment_id: Uuid) -> AppResult<PaymentResponse> {
        let payment = self.repository.find_by_id(payment_id).await?
            .ok_or_else(|| AppError::NotFound("Payment not found".to_string()))?;

        Ok(PaymentResponse::from(payment))
    }

    /// Get payments for account
    pub async fn get_payments_for_account(
        &self,
        account_id: AccountId,
        page: u32,
        limit: u32,
    ) -> AppResult<Vec<PaymentResponse>> {
        let payments = self.repository.find_by_account_id(account_id, page, limit).await?;
        Ok(payments.into_iter().map(PaymentResponse::from).collect())
    }

    /// Cancel payment
    pub async fn cancel_payment(&self, payment_id: Uuid) -> AppResult<()> {
        self.repository.update_status(payment_id, PaymentStatus::Cancelled).await
    }
}
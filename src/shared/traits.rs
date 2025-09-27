use async_trait::async_trait;
use crate::core::error::AppResult;
use uuid::Uuid;

/// Generic repository trait for CRUD operations
#[async_trait]
pub trait Repository<T, ID> {
    async fn create(&self, entity: T) -> AppResult<T>;
    async fn find_by_id(&self, id: ID) -> AppResult<Option<T>>;
    async fn update(&self, id: ID, entity: T) -> AppResult<T>;
    async fn delete(&self, id: ID) -> AppResult<()>;
    async fn find_all(&self, page: u32, limit: u32) -> AppResult<Vec<T>>;
}

/// Service trait for business logic operations
#[async_trait]
pub trait Service<T, CreateDto, UpdateDto, ID> {
    async fn create(&self, dto: CreateDto) -> AppResult<T>;
    async fn get_by_id(&self, id: ID) -> AppResult<T>;
    async fn update(&self, id: ID, dto: UpdateDto) -> AppResult<T>;
    async fn delete(&self, id: ID) -> AppResult<()>;
}

/// Audit trail trait for tracking changes
#[async_trait]
pub trait Auditable {
    async fn log_action(&self, user_id: Uuid, action: &str, entity_type: &str, entity_id: Uuid) -> AppResult<()>;
}

/// Event publisher trait for domain events
#[async_trait]
pub trait EventPublisher {
    async fn publish<E>(&self, event: E) -> AppResult<()>
    where
        E: Send + Sync + 'static;
}
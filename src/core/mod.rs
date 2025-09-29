pub mod audit;
pub mod config;
pub mod database;
pub mod error;
pub mod extractors;
pub mod middleware;
pub mod rate_limit;
pub mod rbac;
pub mod response;
pub mod security;

use crate::core::{
    audit::AuditLogger, rate_limit::RateLimiter, rbac::RbacService,
    security::AccountSecurityService,
};
use mongodb::Client as MongoClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgPool,
    pub mongodb: MongoClient,
    pub config: config::Config,
    pub audit_logger: AuditLogger,
    pub security_service: AccountSecurityService,
    pub rbac_service: RbacService,
    pub rate_limiter: RateLimiter,
}

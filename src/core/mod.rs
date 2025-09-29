pub mod config;
pub mod database;
pub mod error;
pub mod extractors;
pub mod response;

use mongodb::Client as MongoClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgPool,
    pub mongodb: MongoClient,
    pub config: config::Config,
}

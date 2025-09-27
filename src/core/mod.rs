pub mod config;
pub mod database;
pub mod error;

use mongodb::Client as MongoClient;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub postgres: PgPool,
    pub mongodb: MongoClient,
    pub config: config::Config,
}

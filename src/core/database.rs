use mongodb::{options::ClientOptions, Client as MongoClient};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::info;

/// Initialize PostgreSQL connection pool
pub async fn init_postgres(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Connecting to PostgreSQL database...");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("PostgreSQL connection pool created and migrations run successfully");
    Ok(pool)
}

/// Initialize PostgreSQL connection pool with error handling for development
pub async fn init_postgres_safe(database_url: &str) -> Result<PgPool, Box<dyn std::error::Error>> {
    match init_postgres(database_url).await {
        Ok(pool) => Ok(pool),
        Err(e) => {
            tracing::warn!(
                "Failed to connect to PostgreSQL: {}. Using mock pool for development.",
                e
            );
            Err(Box::new(e))
        }
    }
}

/// Initialize MongoDB client
pub async fn init_mongodb(mongodb_url: &str) -> Result<MongoClient, mongodb::error::Error> {
    info!("Connecting to MongoDB...");

    let client_options = ClientOptions::parse(mongodb_url).await?;
    let client = MongoClient::with_options(client_options)?;

    // Verify connection
    client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1}, None)
        .await?;

    info!("MongoDB connection established successfully");
    Ok(client)
}

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub mongodb_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: u64,
    pub database_max_connections: u32,
    pub database_min_connections: u32,
    pub bcrypt_cost: u32,
    pub rate_limit_requests_per_minute: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok(); // Load .env file if present

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://username:password@localhost:5432/openbank".to_string()),
            mongodb_url: env::var("MONGODB_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017/openbank_logs".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            bcrypt_cost: env::var("BCRYPT_COST")
                .unwrap_or_else(|_| "12".to_string())
                .parse()?,
            rate_limit_requests_per_minute: env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
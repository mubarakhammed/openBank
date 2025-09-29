use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Database Configuration
    pub database_url: String,
    pub mongodb_url: String,
    pub mongodb_audit_url: String,

    // Server Configuration
    pub host: String,
    pub port: u16,

    // JWT Configuration
    pub jwt_secret: String,
    pub jwt_expiration: u64,

    // Database Pool Configuration
    pub database_max_connections: u32,
    pub database_min_connections: u32,
    pub bcrypt_cost: u32,

    // Rate Limiting Configuration
    pub rate_limit_requests_per_minute: u64,
    pub rate_limit_burst_size: u32,
    pub rate_limit_window_seconds: u64,

    // Account Security Configuration
    pub max_failed_attempts: i32,
    pub account_lockout_duration_minutes: i64,
    pub progressive_lockout_enabled: bool,
    pub suspicious_activity_threshold: i32,
    pub password_history_count: usize,
    pub require_password_change_days: i64,

    // Audit & Compliance Configuration
    pub audit_log_retention_days: u32,
    pub security_event_log_level: String,
    pub compliance_mode_enabled: bool,

    // RBAC Configuration
    pub default_user_role: String,
    pub role_inheritance_enabled: bool,
    pub custom_permissions_enabled: bool,

    // Monitoring & Alerts Configuration
    pub security_alerts_enabled: bool,
    pub performance_monitoring_enabled: bool,
    pub real_time_threats_enabled: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok(); // Load .env file if present

        Ok(Config {
            // Database Configuration
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgresql://username:password@localhost:5432/openbank".to_string()
            }),
            mongodb_url: env::var("MONGODB_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017/openbank_logs".to_string()),
            mongodb_audit_url: env::var("MONGODB_AUDIT_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017/openbank_audit".to_string()),

            // Server Configuration
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,

            // JWT Configuration
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default-secret-change-in-production".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,

            // Database Pool Configuration
            database_max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            database_min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            bcrypt_cost: env::var("BCRYPT_COST")
                .unwrap_or_else(|_| "12".to_string())
                .parse()?,

            // Rate Limiting Configuration
            rate_limit_requests_per_minute: env::var("RATE_LIMIT_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            rate_limit_burst_size: env::var("RATE_LIMIT_BURST_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            rate_limit_window_seconds: env::var("RATE_LIMIT_WINDOW_SECONDS")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,

            // Account Security Configuration
            max_failed_attempts: env::var("MAX_FAILED_ATTEMPTS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            account_lockout_duration_minutes: env::var("ACCOUNT_LOCKOUT_DURATION_MINUTES")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            progressive_lockout_enabled: env::var("PROGRESSIVE_LOCKOUT_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            suspicious_activity_threshold: env::var("SUSPICIOUS_ACTIVITY_THRESHOLD")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,
            password_history_count: env::var("PASSWORD_HISTORY_COUNT")
                .unwrap_or_else(|_| "12".to_string())
                .parse()?,
            require_password_change_days: env::var("REQUIRE_PASSWORD_CHANGE_DAYS")
                .unwrap_or_else(|_| "90".to_string())
                .parse()?,

            // Audit & Compliance Configuration
            audit_log_retention_days: env::var("AUDIT_LOG_RETENTION_DAYS")
                .unwrap_or_else(|_| "2555".to_string())
                .parse()?,
            security_event_log_level: env::var("SECURITY_EVENT_LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            compliance_mode_enabled: env::var("COMPLIANCE_MODE_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,

            // RBAC Configuration
            default_user_role: env::var("DEFAULT_USER_ROLE")
                .unwrap_or_else(|_| "developer".to_string()),
            role_inheritance_enabled: env::var("ROLE_INHERITANCE_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            custom_permissions_enabled: env::var("CUSTOM_PERMISSIONS_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,

            // Monitoring & Alerts Configuration
            security_alerts_enabled: env::var("SECURITY_ALERTS_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            performance_monitoring_enabled: env::var("PERFORMANCE_MONITORING_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
            real_time_threats_enabled: env::var("REAL_TIME_THREATS_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

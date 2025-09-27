/// API version
pub const API_VERSION: &str = "v1";

/// Default currency
pub const DEFAULT_CURRENCY: &str = "USD";

/// Default pagination limit
pub const DEFAULT_PAGE_LIMIT: u32 = 20;

/// Maximum pagination limit
pub const MAX_PAGE_LIMIT: u32 = 100;

/// JWT token expiration time in seconds (1 hour)
pub const JWT_EXPIRATION_SECONDS: u64 = 3600;

/// Password minimum length
pub const PASSWORD_MIN_LENGTH: usize = 8;

/// Account number length
pub const ACCOUNT_NUMBER_LENGTH: usize = 10;

/// Virtual account number prefix
pub const VIRTUAL_ACCOUNT_PREFIX: &str = "VA";

/// Transaction reference prefix
pub const TRANSACTION_REF_PREFIX: &str = "TXN";

/// Maximum transaction amount (in cents) - $1M
pub const MAX_TRANSACTION_AMOUNT: i64 = 100_000_000;

/// Minimum transaction amount (in cents) - $0.01
pub const MIN_TRANSACTION_AMOUNT: i64 = 1;

/// Rate limiting
pub const DEFAULT_RATE_LIMIT: u64 = 60; // requests per minute

/// Database table names
pub mod tables {
    pub const USERS: &str = "users";
    pub const ACCOUNTS: &str = "accounts";
    pub const TRANSACTIONS: &str = "transactions";
    pub const VIRTUAL_ACCOUNTS: &str = "virtual_accounts";
    pub const BALANCES: &str = "balances";
    pub const PAYMENTS: &str = "payments";
    pub const IDENTITY_VERIFICATIONS: &str = "identity_verifications";
    pub const INCOME_VERIFICATIONS: &str = "income_verifications";
}

/// MongoDB collection names
pub mod collections {
    pub const LOGS: &str = "logs";
    pub const ANALYTICS: &str = "analytics";
    pub const AUDIT_TRAIL: &str = "audit_trail";
    pub const NOTIFICATIONS: &str = "notifications";
}
/// OpenBank API Scopes
/// 
/// This module defines all available scopes for the OpenBank API.
/// Scopes are based on the actual banking modules.

// Core Banking Modules
pub const IDENTITY: &str = "identity";
pub const INCOME: &str = "income";
pub const PAYMENTS: &str = "payments";
pub const TRANSACTIONS: &str = "transactions";
pub const USER_DATA: &str = "user-data";
pub const VIRTUAL_ACCOUNTS: &str = "virtual-accounts";

/// Default scope sets for different project types
pub struct ScopeSets;

impl ScopeSets {
    /// Basic banking operations
    pub fn basic() -> Vec<String> {
        vec![
            TRANSACTIONS.to_string(),
            USER_DATA.to_string(),
        ]
    }

    /// Standard banking app scopes
    pub fn banking_app() -> Vec<String> {
        vec![
            TRANSACTIONS.to_string(),
            PAYMENTS.to_string(),
            USER_DATA.to_string(),
        ]
    }

    /// Fintech platform scopes (includes virtual accounts)
    pub fn fintech_platform() -> Vec<String> {
        vec![
            VIRTUAL_ACCOUNTS.to_string(),
            TRANSACTIONS.to_string(),
            PAYMENTS.to_string(),
            USER_DATA.to_string(),
        ]
    }

    /// Identity verification service scopes
    pub fn identity_service() -> Vec<String> {
        vec![
            IDENTITY.to_string(),
            USER_DATA.to_string(),
        ]
    }

    /// Income verification service scopes
    pub fn income_service() -> Vec<String> {
        vec![
            INCOME.to_string(),
            USER_DATA.to_string(),
        ]
    }

    /// Full access to all modules
    pub fn full_access() -> Vec<String> {
        vec![
            IDENTITY.to_string(),
            INCOME.to_string(),
            PAYMENTS.to_string(),
            TRANSACTIONS.to_string(),
            USER_DATA.to_string(),
            VIRTUAL_ACCOUNTS.to_string(),
        ]
    }
}

/// Validates if a scope is valid
pub fn is_valid_scope(scope: &str) -> bool {
    matches!(scope,
        IDENTITY | INCOME | PAYMENTS | TRANSACTIONS | USER_DATA | VIRTUAL_ACCOUNTS
    )
}

/// Get all available scopes
pub fn all_scopes() -> Vec<String> {
    vec![
        IDENTITY.to_string(),
        INCOME.to_string(),
        PAYMENTS.to_string(),
        TRANSACTIONS.to_string(),
        USER_DATA.to_string(),
        VIRTUAL_ACCOUNTS.to_string(),
    ]
}

/// Scope descriptions for documentation
pub fn get_scope_description(scope: &str) -> Option<&'static str> {
    match scope {
        IDENTITY => Some("Access to identity verification and management features"),
        INCOME => Some("Access to income verification and analysis features"),
        PAYMENTS => Some("Access to payment processing and management features"),
        TRANSACTIONS => Some("Access to transaction management and history features"),
        USER_DATA => Some("Access to user profile and account data features"),
        VIRTUAL_ACCOUNTS => Some("Access to virtual account creation and management features"),
        _ => None,
    }
}
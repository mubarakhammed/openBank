use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::collections::HashMap;
use crate::core::error::AppResult;

/// Account security tracking model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AccountSecurity {
    pub id: Uuid,
    pub developer_id: Uuid,
    
    // Failed login attempts
    pub failed_attempts: i32,
    pub last_failed_attempt: Option<DateTime<Utc>>,
    
    // Account lockout
    pub locked_until: Option<DateTime<Utc>>,
    pub lock_reason: Option<String>,
    
    // Login tracking
    pub last_successful_login: Option<DateTime<Utc>>,
    pub login_count: i64,
    
    // Suspicious activity flags
    pub suspicious_activity_score: i32,
    pub suspicious_ips: Vec<String>,
    
    // Password security
    pub password_last_changed: DateTime<Utc>,
    pub password_history_hashes: Vec<String>, // Store last 12 password hashes
    
    // MFA settings (for future implementation)
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub backup_codes: Vec<String>,
    
    // Security preferences
    pub security_notifications: bool,
    pub login_alerts: bool,
    
    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AccountSecurity {
    pub fn new(developer_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            developer_id,
            failed_attempts: 0,
            last_failed_attempt: None,
            locked_until: None,
            lock_reason: None,
            last_successful_login: None,
            login_count: 0,
            suspicious_activity_score: 0,
            suspicious_ips: Vec::new(),
            password_last_changed: now,
            password_history_hashes: Vec::new(),
            mfa_enabled: false,
            mfa_secret: None,
            backup_codes: Vec::new(),
            security_notifications: true,
            login_alerts: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if account is currently locked
    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            Utc::now() < locked_until
        } else {
            false
        }
    }

    /// Get remaining lock time
    pub fn lock_remaining(&self) -> Option<Duration> {
        if let Some(locked_until) = self.locked_until {
            let now = Utc::now();
            if now < locked_until {
                Some(locked_until - now)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Check if account needs security attention
    pub fn needs_security_review(&self) -> bool {
        self.suspicious_activity_score > 50 
            || self.failed_attempts > 3
            || self.password_last_changed < Utc::now() - Duration::days(90)
    }
}

/// Security event for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub timestamp: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    LoginAttempt,
    PasswordChange,
    AccountLock,
    AccountUnlock,
    SuspiciousActivity,
    MfaAttempt,
}

/// Account security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub max_failed_attempts: i32,
    pub lockout_duration_minutes: i64,
    pub progressive_lockout: bool,
    pub suspicious_activity_threshold: i32,
    pub password_history_count: usize,
    pub require_password_change_days: i64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_minutes: 30,
            progressive_lockout: true,
            suspicious_activity_threshold: 50,
            password_history_count: 12,
            require_password_change_days: 90,
        }
    }
}

/// Account security service
#[derive(Clone)]
pub struct AccountSecurityService {
    config: SecurityConfig,
}

impl AccountSecurityService {
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }

    /// Record a failed login attempt
    pub fn record_failed_attempt(
        &self,
        security: &mut AccountSecurity,
        ip_address: String,
    ) -> AppResult<SecurityAction> {
        security.failed_attempts += 1;
        security.last_failed_attempt = Some(Utc::now());
        security.updated_at = Utc::now();

        // Add to suspicious IPs if multiple failures
        if security.failed_attempts >= 3 && !security.suspicious_ips.contains(&ip_address) {
            security.suspicious_ips.push(ip_address.clone());
            security.suspicious_activity_score += 10;
        }

        // Determine if account should be locked
        if security.failed_attempts >= self.config.max_failed_attempts {
            let lock_duration = if self.config.progressive_lockout {
                // Progressive lockout: increase duration with each lockout
                let base_minutes = self.config.lockout_duration_minutes;
                let multiplier = (security.failed_attempts - self.config.max_failed_attempts) + 1;
                Duration::minutes(base_minutes * multiplier as i64)
            } else {
                Duration::minutes(self.config.lockout_duration_minutes)
            };

            security.locked_until = Some(Utc::now() + lock_duration);
            security.lock_reason = Some(format!(
                "Account locked due to {} consecutive failed login attempts",
                security.failed_attempts
            ));

            return Ok(SecurityAction::AccountLocked {
                duration: lock_duration,
                reason: security.lock_reason.clone().unwrap(),
            });
        }

        Ok(SecurityAction::IncrementFailures {
            current_count: security.failed_attempts,
            max_attempts: self.config.max_failed_attempts,
        })
    }

    /// Record a successful login
    pub fn record_successful_login(
        &self,
        security: &mut AccountSecurity,
        ip_address: String,
    ) -> AppResult<()> {
        security.failed_attempts = 0;
        security.last_failed_attempt = None;
        security.last_successful_login = Some(Utc::now());
        security.login_count += 1;
        security.updated_at = Utc::now();

        // Remove IP from suspicious list if login successful
        security.suspicious_ips.retain(|ip| ip != &ip_address);
        
        // Reduce suspicious activity score on successful login
        security.suspicious_activity_score = (security.suspicious_activity_score - 5).max(0);

        Ok(())
    }

    /// Check if password can be changed (not in history)
    pub fn can_use_password(&self, security: &AccountSecurity, password_hash: &str) -> bool {
        !security.password_history_hashes.contains(&password_hash.to_string())
    }

    /// Record password change
    pub fn record_password_change(
        &self,
        security: &mut AccountSecurity,
        new_password_hash: String,
    ) -> AppResult<()> {
        // Add old password to history
        if !security.password_history_hashes.is_empty() {
            security.password_history_hashes.insert(0, new_password_hash.clone());
        }

        // Keep only configured number of password history
        security.password_history_hashes.truncate(self.config.password_history_count);

        security.password_last_changed = Utc::now();
        security.updated_at = Utc::now();

        // Reset failed attempts and unlock account if locked
        security.failed_attempts = 0;
        security.locked_until = None;
        security.lock_reason = None;

        Ok(())
    }

    /// Check for suspicious activity patterns
    pub fn detect_suspicious_activity(
        &self,
        security: &AccountSecurity,
        ip_address: &str,
        _user_agent: Option<&str>,
    ) -> SuspiciousActivityLevel {
        let mut risk_score = 0;

        // Check for suspicious IP
        if security.suspicious_ips.contains(&ip_address.to_string()) {
            risk_score += 20;
        }

        // Multiple failed attempts recently
        if security.failed_attempts > 2 {
            risk_score += security.failed_attempts * 5;
        }

        // Account has high suspicious activity score
        risk_score += security.suspicious_activity_score;

        // Check for unusual patterns (future enhancement)
        // - Login from new country/timezone
        // - Unusual user agent
        // - Rapid repeated requests

        match risk_score {
            0..=20 => SuspiciousActivityLevel::Low,
            21..=50 => SuspiciousActivityLevel::Medium,
            51..=80 => SuspiciousActivityLevel::High,
            _ => SuspiciousActivityLevel::Critical,
        }
    }

    /// Unlock account manually (admin function)
    pub fn unlock_account(&self, security: &mut AccountSecurity, reason: String) -> AppResult<()> {
        security.locked_until = None;
        security.lock_reason = None;
        security.failed_attempts = 0;
        security.suspicious_activity_score = 0;
        security.updated_at = Utc::now();

        tracing::info!(
            developer_id = %security.developer_id,
            reason = reason,
            "Account manually unlocked"
        );

        Ok(())
    }
}

/// Actions taken by security service
#[derive(Debug, Clone)]
pub enum SecurityAction {
    AccountLocked {
        duration: Duration,
        reason: String,
    },
    IncrementFailures {
        current_count: i32,
        max_attempts: i32,
    },
    SuspiciousActivityDetected {
        level: SuspiciousActivityLevel,
        actions_taken: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SuspiciousActivityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Password policy checker
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
    pub forbidden_passwords: Vec<String>,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            forbidden_passwords: vec![
                "password".to_string(),
                "123456".to_string(),
                "password123".to_string(),
                "admin".to_string(),
                "qwerty".to_string(),
                "letmein".to_string(),
            ],
        }
    }
}

impl PasswordPolicy {
    pub fn validate(&self, password: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check length
        if password.len() < self.min_length {
            errors.push(format!("Password must be at least {} characters long", self.min_length));
        }

        // Check character requirements
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }

        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }

        if self.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            errors.push("Password must contain at least one number".to_string());
        }

        if self.require_special_chars && !password.chars().any(|c| !c.is_alphanumeric()) {
            errors.push("Password must contain at least one special character".to_string());
        }

        // Check forbidden passwords
        let password_lower = password.to_lowercase();
        for forbidden in &self.forbidden_passwords {
            if password_lower.contains(&forbidden.to_lowercase()) {
                errors.push("Password contains common/forbidden words".to_string());
                break;
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
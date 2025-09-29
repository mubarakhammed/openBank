use chrono::{DateTime, Utc};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{error, info};
use uuid::Uuid;

/// Audit event types for authentication and authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    // Authentication Events
    LoginAttempt,
    LoginSuccess,
    LoginFailure,
    Logout,
    TokenGenerated,
    TokenRefreshed,
    TokenRevoked,
    TokenValidated,
    TokenExpired,

    // Authorization Events
    AccessGranted,
    AccessDenied,
    ScopeValidated,
    ScopeViolation,

    // Account Management
    DeveloperRegistered,
    DeveloperUpdated,
    ProjectCreated,
    ProjectUpdated,
    ProjectDeactivated,

    // Security Events
    RateLimitExceeded,
    SuspiciousActivity,
    AccountLocked,
    AccountUnlocked,
    PasswordChanged,
    MfaEnabled,
    MfaDisabled,

    // System Events
    ConfigurationChanged,
    DatabaseAccess,
    ApiAccess,

    // Compliance Events
    DataExported,
    DataDeleted,
    ConsentGranted,
    ConsentRevoked,
}

/// Audit event severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Complete audit event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Unique event identifier
    pub id: Uuid,

    /// Event type
    pub event_type: AuditEventType,

    /// Event severity
    pub severity: AuditSeverity,

    /// Timestamp when event occurred
    pub timestamp: DateTime<Utc>,

    /// User/Developer who performed the action (if applicable)
    pub user_id: Option<Uuid>,

    /// Project context (if applicable)
    pub project_id: Option<Uuid>,

    /// Client IP address
    pub ip_address: String,

    /// User agent string
    pub user_agent: Option<String>,

    /// Request ID for correlation
    pub request_id: Option<String>,

    /// Session ID (if applicable)
    pub session_id: Option<String>,

    /// Whether the action was successful
    pub success: bool,

    /// Error message (if failed)
    pub error_message: Option<String>,

    /// Additional structured metadata
    pub metadata: HashMap<String, Value>,

    /// Resource accessed
    pub resource: Option<String>,

    /// Action performed
    pub action: Option<String>,

    /// Before/after values for changes
    pub changes: Option<Value>,

    /// Compliance related fields
    pub compliance_tags: Vec<String>,

    /// Risk score (0-100)
    pub risk_score: Option<u8>,
}

impl AuditEvent {
    pub fn new(event_type: AuditEventType) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            severity: AuditSeverity::Info,
            timestamp: Utc::now(),
            user_id: None,
            project_id: None,
            ip_address: String::new(),
            user_agent: None,
            request_id: None,
            session_id: None,
            success: true,
            error_message: None,
            metadata: HashMap::new(),
            resource: None,
            action: None,
            changes: None,
            compliance_tags: Vec::new(),
            risk_score: None,
        }
    }

    /// Builder pattern methods
    pub fn severity(mut self, severity: AuditSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn project_id(mut self, project_id: Uuid) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn ip_address(mut self, ip: String) -> Self {
        self.ip_address = ip;
        self
    }

    pub fn user_agent(mut self, agent: String) -> Self {
        self.user_agent = Some(agent);
        self
    }

    pub fn request_id(mut self, id: String) -> Self {
        self.request_id = Some(id);
        self
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn error(mut self, error: String) -> Self {
        self.error_message = Some(error);
        self.success = false;
        self.severity = AuditSeverity::Error;
        self
    }

    pub fn metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn resource(mut self, resource: String) -> Self {
        self.resource = Some(resource);
        self
    }

    pub fn action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }

    pub fn compliance_tag(mut self, tag: String) -> Self {
        self.compliance_tags.push(tag);
        self
    }

    pub fn risk_score(mut self, score: u8) -> Self {
        self.risk_score = Some(score.min(100));
        self
    }
}

/// Audit logger service
#[derive(Clone)]
pub struct AuditLogger {
    collection: Collection<AuditEvent>,
}

impl AuditLogger {
    pub fn new(mongodb_client: Client) -> Self {
        let db = mongodb_client.database("openbank_audit");
        let collection = db.collection::<AuditEvent>("audit_events");

        Self { collection }
    }

    /// Log an audit event
    pub async fn log(&self, event: AuditEvent) {
        info!(
            event_id = %event.id,
            event_type = ?event.event_type,
            user_id = ?event.user_id,
            success = event.success,
            "Audit event logged"
        );

        match self.collection.insert_one(&event, None).await {
            Ok(_) => {
                info!(event_id = %event.id, "Audit event stored in database");
            }
            Err(e) => {
                error!(
                    event_id = %event.id,
                    error = %e,
                    "Failed to store audit event in database"
                );
            }
        }
    }

    /// Log authentication attempt
    pub async fn log_auth_attempt(&self, user_id: Option<Uuid>, ip: String, success: bool) {
        let event = AuditEvent::new(AuditEventType::LoginAttempt)
            .ip_address(ip)
            .success(success)
            .compliance_tag("SOC2".to_string())
            .compliance_tag("PCI_DSS".to_string());

        let event = if let Some(id) = user_id {
            event.user_id(id)
        } else {
            event
        };

        let event = if !success {
            event.severity(AuditSeverity::Warning).risk_score(30)
        } else {
            event.risk_score(0)
        };

        self.log(event).await;
    }

    /// Log token generation
    pub async fn log_token_generated(
        &self,
        user_id: Uuid,
        project_id: Uuid,
        scopes: Vec<String>,
        ip: String,
    ) {
        let event = AuditEvent::new(AuditEventType::TokenGenerated)
            .user_id(user_id)
            .project_id(project_id)
            .ip_address(ip)
            .metadata("scopes".to_string(), serde_json::to_value(scopes).unwrap())
            .compliance_tag("OAuth2".to_string())
            .risk_score(10);

        self.log(event).await;
    }

    /// Log access denied
    pub async fn log_access_denied(&self, resource: String, reason: String, ip: String) {
        let event = AuditEvent::new(AuditEventType::AccessDenied)
            .severity(AuditSeverity::Warning)
            .resource(resource)
            .ip_address(ip)
            .error(reason)
            .risk_score(25)
            .compliance_tag("RBAC".to_string());

        self.log(event).await;
    }

    /// Log rate limit exceeded
    pub async fn log_rate_limit_exceeded(&self, ip: String, requests_count: u32) {
        let event = AuditEvent::new(AuditEventType::RateLimitExceeded)
            .severity(AuditSeverity::Warning)
            .ip_address(ip)
            .metadata(
                "requests_count".to_string(),
                serde_json::to_value(requests_count).unwrap(),
            )
            .risk_score(40)
            .compliance_tag("SECURITY".to_string());

        self.log(event).await;
    }

    /// Log suspicious activity
    pub async fn log_suspicious_activity(
        &self,
        user_id: Option<Uuid>,
        ip: String,
        activity_type: String,
        details: HashMap<String, Value>,
    ) {
        let mut event = AuditEvent::new(AuditEventType::SuspiciousActivity)
            .severity(AuditSeverity::Critical)
            .ip_address(ip)
            .action(activity_type)
            .risk_score(80)
            .compliance_tag("FRAUD_DETECTION".to_string());

        if let Some(id) = user_id {
            event = event.user_id(id);
        }

        for (key, value) in details {
            event = event.metadata(key, value);
        }

        self.log(event).await;
    }

    /// Log GDPR data access
    pub async fn log_gdpr_data_access(&self, user_id: Uuid, data_type: String, purpose: String) {
        let event = AuditEvent::new(AuditEventType::ApiAccess)
            .user_id(user_id)
            .resource(data_type.clone())
            .action("data_access".to_string())
            .metadata(
                "purpose".to_string(),
                serde_json::to_value(purpose).unwrap(),
            )
            .metadata(
                "data_type".to_string(),
                serde_json::to_value(data_type).unwrap(),
            )
            .compliance_tag("GDPR".to_string())
            .risk_score(5);

        self.log(event).await;
    }

    /// Query audit events for compliance reporting
    pub async fn get_compliance_report(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        compliance_tag: Option<String>,
        user_id: Option<Uuid>,
    ) -> Result<Vec<AuditEvent>, mongodb::error::Error> {
        use mongodb::bson::doc;

        let mut filter = doc! {
            "timestamp": {
                "$gte": start_date.to_rfc3339(),
                "$lte": end_date.to_rfc3339()
            }
        };

        if let Some(tag) = compliance_tag {
            filter.insert("compliance_tags", tag);
        }

        if let Some(id) = user_id {
            filter.insert("user_id", id.to_string());
        }

        let mut cursor = self.collection.find(filter, None).await?;
        let mut results = Vec::new();
        while cursor.advance().await? {
            results.push(cursor.deserialize_current()?);
        }
        Ok(results)
    }
}

/// Middleware to extract request context for audit logging
pub fn extract_audit_context(req: &axum::extract::Request) -> AuditContext {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .or_else(|| req.headers().get("x-real-ip"))
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    AuditContext {
        ip_address: ip,
        user_agent,
        request_id,
        method: req.method().to_string(),
        uri: req.uri().to_string(),
    }
}

#[derive(Debug)]
pub struct AuditContext {
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,
    pub method: String,
    pub uri: String,
}

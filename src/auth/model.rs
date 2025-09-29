use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Developer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub company: Option<String>,
    pub title: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectEnvironment {
    Development,
    Staging,
    Production,
}

impl sqlx::Type<sqlx::Postgres> for ProjectEnvironment {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for ProjectEnvironment {
    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer) -> sqlx::encode::IsNull {
        let s = match self {
            ProjectEnvironment::Development => "development",
            ProjectEnvironment::Staging => "staging",
            ProjectEnvironment::Production => "production",
        };
        <&str as sqlx::Encode<sqlx::Postgres>>::encode(s, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for ProjectEnvironment {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s {
            "development" => Ok(ProjectEnvironment::Development),
            "staging" => Ok(ProjectEnvironment::Staging),
            "production" => Ok(ProjectEnvironment::Production),
            _ => Err(format!("Invalid ProjectEnvironment: {}", s).into()),
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub developer_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub environment: ProjectEnvironment,
    pub client_id: String,
    pub client_secret_hash: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OAuthToken {
    pub id: Uuid,
    pub project_id: Uuid,
    pub developer_id: Uuid,
    pub access_token_hash: String,
    pub token_type: String,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub jti: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDeveloperRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 100))]
    pub company: Option<String>,
    #[validate(length(max = 100))]
    pub title: Option<String>,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub environment: ProjectEnvironment,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub jti: String, // Token identifier to refresh
}

#[derive(Debug, Serialize)]
pub struct DeveloperResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub company: Option<String>,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub environment: ProjectEnvironment,
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub developer_id: Uuid,
    pub project_id: Uuid,
    pub scopes: Vec<String>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
    pub developer_id: Uuid,
    pub project_id: Uuid,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl From<Developer> for DeveloperResponse {
    fn from(developer: Developer) -> Self {
        Self {
            id: developer.id,
            name: developer.name,
            email: developer.email,
            company: developer.company,
            title: developer.title,
            created_at: developer.created_at,
        }
    }
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            name: project.name,
            description: project.description,
            environment: project.environment,
            client_id: project.client_id,
            redirect_uris: project.redirect_uris,
            scopes: project.scopes,
            is_active: project.is_active,
            created_at: project.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeInfo {
    pub scope: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeSetsInfo {
    pub basic: Vec<String>,
    pub banking_app: Vec<String>,
    pub fintech_platform: Vec<String>,
    pub identity_service: Vec<String>,
    pub income_service: Vec<String>,
    pub full_access: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesResponse {
    pub scopes: Vec<ScopeInfo>,
    pub scope_sets: ScopeSetsInfo,
}

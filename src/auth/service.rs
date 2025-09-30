use super::model::*;
use super::repository::AuthRepository;
use super::scopes;
use crate::core::error::{AppError, AppResult};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    pub repository: AuthRepository,
    pub jwt_secret: String,
}

impl AuthService {
    pub fn new(repository: AuthRepository, jwt_secret: String) -> Self {
        Self {
            repository,
            jwt_secret,
        }
    }

    pub async fn register_developer(
        &self,
        request: RegisterDeveloperRequest,
    ) -> AppResult<DeveloperResponse> {
        if let Some(_) = self
            .repository
            .find_developer_by_email(&request.email)
            .await?
        {
            return Err(AppError::Validation("Email already exists".to_string()));
        }

        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|_| AppError::Internal("Failed to hash password".to_string()))?;

        let developer = self
            .repository
            .create_developer(
                &request.name,
                &request.email,
                request.company.as_deref(),
                request.title.as_deref(),
                &password_hash,
            )
            .await?;
        Ok(DeveloperResponse::from(developer))
    }

    pub async fn login_developer(&self, request: LoginRequest) -> AppResult<LoginResponse> {
        // Find developer by email
        let developer = self
            .repository
            .find_developer_by_email(&request.email)
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

        // Verify password
        if !verify(&request.password, &developer.password_hash)
            .map_err(|_| AppError::Internal("Password verification failed".to_string()))?
        {
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        // Generate access token for developer session
        let expires_at = Utc::now() + chrono::Duration::hours(24); // 24 hour session
        let jti = Uuid::new_v4().to_string();

        let claims = JwtClaims {
            iss: "openbank-auth".to_string(),
            aud: "openbank-dashboard".to_string(),
            sub: developer.id.to_string(),
            exp: expires_at.timestamp(),
            iat: Utc::now().timestamp(),
            jti: jti.clone(),
            developer_id: developer.id,
            project_id: Uuid::nil(), // Use nil UUID for developer login (no specific project)
            scopes: vec!["developer".to_string()], // Developer scope for dashboard access
        };

        let access_token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

        let developer_response = DeveloperResponse::from(developer);

        Ok(LoginResponse {
            developer: developer_response,
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: 86400, // 24 hours in seconds
        })
    }

    pub async fn create_project(
        &self,
        developer_id: Uuid,
        request: CreateProjectRequest,
    ) -> AppResult<ProjectResponse> {
        // Validate requested scopes
        self.validate_project_scopes(&request.scopes)?;

        let client_id = self.generate_client_id();
        let client_secret = self.generate_client_secret();
        let client_secret_hash = hash(&client_secret, DEFAULT_COST)
            .map_err(|_| AppError::Internal("Failed to hash client secret".to_string()))?;

        let project = self
            .repository
            .create_project(
                developer_id,
                &request.name,
                request.description.as_deref().unwrap_or(""),
                request.environment,
                &client_id,
                &client_secret_hash,
                &request.redirect_uris,
                &request.scopes,
            )
            .await?;

        let mut response = ProjectResponse::from(project);
        response.client_id = format!("{}:{}", client_id, client_secret);
        Ok(response)
    }

    pub async fn handle_client_credentials_flow(
        &self,
        request: TokenRequest,
    ) -> AppResult<TokenResponse> {
        if request.grant_type != "client_credentials" {
            return Err(AppError::Validation("Invalid grant type".to_string()));
        }

        let project = self
            .repository
            .find_project_by_client_id(&request.client_id)
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid client credentials".to_string()))?;

        if !verify(&request.client_secret, &project.client_secret_hash)
            .map_err(|_| AppError::Internal("Failed to verify client secret".to_string()))?
        {
            return Err(AppError::Authentication(
                "Invalid client credentials".to_string(),
            ));
        }

        let requested_scopes = request
            .scope
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_else(|| project.scopes.clone());

        // Validate all requested scopes are valid
        for scope in &requested_scopes {
            if !scopes::is_valid_scope(scope) {
                return Err(AppError::Validation(format!("Invalid scope: {}", scope)));
            }
        }

        // Ensure requested scopes are a subset of project scopes
        for scope in &requested_scopes {
            if !project.scopes.contains(scope) {
                return Err(AppError::Validation(format!(
                    "Scope '{}' not authorized for this project",
                    scope
                )));
            }
        }

        let scopes = requested_scopes;

        // Environment-based token expiration for better developer experience
        let expires_at = match project.environment {
            ProjectEnvironment::Development => Utc::now() + Duration::hours(24), // 24 hours for development
            ProjectEnvironment::Staging => Utc::now() + Duration::hours(8), // 8 hours for staging
            ProjectEnvironment::Production => Utc::now() + Duration::hours(4), // 4 hours for production
        };

        let expires_in_seconds = match project.environment {
            ProjectEnvironment::Development => 24 * 3600, // 24 hours
            ProjectEnvironment::Staging => 8 * 3600,      // 8 hours
            ProjectEnvironment::Production => 4 * 3600,   // 4 hours
        };

        let jti = Uuid::new_v4().to_string();

        let claims = JwtClaims {
            iss: "openbank-auth".to_string(),
            aud: "openbank-api".to_string(),
            sub: project.developer_id.to_string(),
            exp: expires_at.timestamp(),
            iat: Utc::now().timestamp(),
            jti: jti.clone(),
            developer_id: project.developer_id,
            project_id: project.id,
            scopes: scopes.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

        let oauth_token = OAuthToken {
            id: Uuid::new_v4(),
            project_id: project.id,
            developer_id: project.developer_id,
            access_token_hash: self.hash_secret(&token),
            token_type: "Bearer".to_string(),
            scopes: scopes.clone(),
            expires_at,
            jti,
            created_at: Utc::now(),
        };

        self.repository.store_oauth_token(&oauth_token).await?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: expires_in_seconds,
            scope: scopes.join(" "),
        })
    }

    pub async fn refresh_access_token(
        &self,
        request: RefreshTokenRequest,
    ) -> AppResult<TokenResponse> {
        // Verify client credentials
        let project = self
            .repository
            .find_project_by_client_id(&request.client_id)
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid client credentials".to_string()))?;

        if !verify(&request.client_secret, &project.client_secret_hash)
            .map_err(|_| AppError::Internal("Failed to verify client secret".to_string()))?
        {
            return Err(AppError::Authentication(
                "Invalid client credentials".to_string(),
            ));
        }

        // Get existing token by JTI
        let existing_token = self
            .repository
            .get_oauth_token_by_jti(&request.jti)
            .await?
            .ok_or_else(|| AppError::Authentication("Token not found".to_string()))?;

        // Verify token belongs to this project
        if existing_token.project_id != project.id {
            return Err(AppError::Authentication(
                "Token not authorized for this project".to_string(),
            ));
        }

        // Check if token is still valid (not expired)
        if existing_token.expires_at < Utc::now() {
            return Err(AppError::Authentication("Token has expired".to_string()));
        }

        // Generate new token with same scopes but extended expiration
        let expires_at = match project.environment {
            ProjectEnvironment::Development => Utc::now() + Duration::hours(24),
            ProjectEnvironment::Staging => Utc::now() + Duration::hours(8),
            ProjectEnvironment::Production => Utc::now() + Duration::hours(4),
        };

        let expires_in_seconds = match project.environment {
            ProjectEnvironment::Development => 24 * 3600,
            ProjectEnvironment::Staging => 8 * 3600,
            ProjectEnvironment::Production => 4 * 3600,
        };

        let new_jti = Uuid::new_v4().to_string();

        let claims = JwtClaims {
            iss: "openbank-auth".to_string(),
            aud: "openbank-api".to_string(),
            sub: project.developer_id.to_string(),
            exp: expires_at.timestamp(),
            iat: Utc::now().timestamp(),
            jti: new_jti.clone(),
            developer_id: project.developer_id,
            project_id: project.id,
            scopes: existing_token.scopes.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

        let new_token = OAuthToken {
            id: Uuid::new_v4(),
            project_id: project.id,
            developer_id: project.developer_id,
            access_token_hash: self.hash_secret(&token),
            token_type: "Bearer".to_string(),
            scopes: existing_token.scopes.clone(),
            expires_at,
            jti: new_jti,
            created_at: Utc::now(),
        };

        // Store new token and optionally revoke old one
        self.repository.store_oauth_token(&new_token).await?;
        self.repository.revoke_oauth_token(&request.jti).await?;

        Ok(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: expires_in_seconds,
            scope: existing_token.scopes.join(" "),
        })
    }

    pub async fn verify_access_token(&self, token: &str) -> AppResult<MeResponse> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["openbank-api"]);
        validation.set_issuer(&["openbank-auth"]);

        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            tracing::error!("JWT decode error: {}", e);
            AppError::Authentication(format!("Invalid token: {}", e))
        })?;

        let oauth_token = self
            .repository
            .find_oauth_token_by_jti(&token_data.claims.jti)
            .await?
            .ok_or_else(|| {
                tracing::error!("Token not found in database: jti={}", token_data.claims.jti);
                AppError::Authentication("Token not found".to_string())
            })?;

        if oauth_token.expires_at < Utc::now() {
            return Err(AppError::Authentication("Token expired".to_string()));
        }

        Ok(MeResponse {
            developer_id: oauth_token.developer_id,
            project_id: oauth_token.project_id,
            scopes: oauth_token.scopes,
            expires_at: oauth_token.expires_at,
        })
    }

    fn generate_client_id(&self) -> String {
        format!("ck_{}", self.generate_random_string(32))
    }

    fn generate_client_secret(&self) -> String {
        format!("cs_{}", self.generate_random_string(64))
    }

    fn generate_random_string(&self, length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    fn hash_secret(&self, secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Get default scopes for a project based on its type
    pub fn get_default_scopes_for_project(&self, environment: &ProjectEnvironment) -> Vec<String> {
        match environment {
            ProjectEnvironment::Development => scopes::ScopeSets::basic(),
            ProjectEnvironment::Staging => scopes::ScopeSets::banking_app(),
            ProjectEnvironment::Production => scopes::ScopeSets::banking_app(),
        }
    }

    /// Validate that all scopes in the list are valid OpenBank scopes
    pub fn validate_project_scopes(&self, scopes: &[String]) -> AppResult<()> {
        for scope in scopes {
            if !scopes::is_valid_scope(scope) {
                return Err(AppError::Validation(format!(
                    "Invalid scope '{}'. Available scopes: {}",
                    scope,
                    scopes::all_scopes().join(", ")
                )));
            }
        }
        Ok(())
    }
}

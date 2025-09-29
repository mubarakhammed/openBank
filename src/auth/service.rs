use super::model::*;
use super::repository::AuthRepository;
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
            .create_developer(&request.name, &request.email, &password_hash)
            .await?;
        Ok(DeveloperResponse::from(developer))
    }

    pub async fn create_project(
        &self,
        developer_id: Uuid,
        request: CreateProjectRequest,
    ) -> AppResult<ProjectResponse> {
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

        let scopes = request
            .scope
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_else(|| project.scopes.clone());

        let expires_at = Utc::now() + Duration::hours(1);
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
            expires_in: 3600,
            scope: scopes.join(" "),
        })
    }

    pub async fn verify_access_token(&self, token: &str) -> AppResult<MeResponse> {
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AppError::Authentication("Invalid token".to_string()))?;

        let oauth_token = self
            .repository
            .find_oauth_token_by_jti(&token_data.claims.jti)
            .await?
            .ok_or_else(|| AppError::Authentication("Token not found".to_string()))?;

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
}

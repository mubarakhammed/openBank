use super::model::{AuthResponse, Claims, LoginRequest, RegisterRequest, User, UserResponse};
use super::repository::AuthRepository;
use crate::core::error::{AppError, AppResult};
use crate::shared::traits::Repository;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

pub struct AuthService {
    repository: AuthRepository,
    jwt_secret: String,
    jwt_expiration: u64,
}

impl AuthService {
    pub fn new(repository: AuthRepository, jwt_secret: String, jwt_expiration: u64) -> Self {
        Self {
            repository,
            jwt_secret,
            jwt_expiration,
        }
    }

    /// Register a new user
    pub async fn register(&self, request: RegisterRequest) -> AppResult<AuthResponse> {
        // TODO: Implement user registration
        // 1. Check if email already exists
        if self.repository.email_exists(&request.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        // 2. Hash password
        let password_hash = hash(&request.password, DEFAULT_COST)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;

        // 3. Create user entity
        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            email: request.email.clone(),
            password_hash,
            first_name: request.first_name,
            last_name: request.last_name,
            phone: request.phone,
            is_verified: false,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        // 4. Save user to database
        let created_user = self.repository.create(user).await?;

        // 5. Generate JWT token
        let token = self.generate_token(&created_user)?;

        // 6. Return response
        Ok(AuthResponse {
            user: UserResponse::from(created_user),
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_expiration,
        })
    }

    /// Login user
    pub async fn login(&self, request: LoginRequest) -> AppResult<AuthResponse> {
        // TODO: Implement user login
        // 1. Find user by email
        let user = self
            .repository
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

        // 2. Verify password
        let is_valid = verify(&request.password, &user.password_hash)
            .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        // 3. Check if user is active
        if !user.is_active {
            return Err(AppError::Authentication(
                "Account is deactivated".to_string(),
            ));
        }

        // 4. Generate JWT token
        let token = self.generate_token(&user)?;

        // 5. Return response
        Ok(AuthResponse {
            user: UserResponse::from(user),
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_expiration,
        })
    }

    /// Generate JWT token
    fn generate_token(&self, user: &User) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.jwt_expiration as i64);

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| AppError::Internal(format!("Token generation failed: {}", e)))
    }

    /// Verify JWT token
    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        // TODO: Implement token verification
        // Use jsonwebtoken::decode to verify and decode the token
        // Check expiration, signature, etc.

        // Placeholder implementation
        Err(AppError::Authentication(
            "Token verification not implemented".to_string(),
        ))
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: Uuid) -> AppResult<UserResponse> {
        let user = self
            .repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserResponse::from(user))
    }
}

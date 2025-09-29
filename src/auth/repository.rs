use crate::auth::model::{Developer, OAuthToken, Project, ProjectEnvironment};
use crate::core::error::AppResult;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthRepository {
    pub pool: PgPool,
}

impl AuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_developer(
        &self,
        name: &str,
        email: &str,
        company: Option<&str>,
        title: Option<&str>,
        password_hash: &str,
    ) -> AppResult<Developer> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let developer = sqlx::query_as::<_, Developer>(
            "INSERT INTO developers (id, name, email, company, title, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, name, email, company, title, password_hash, created_at, updated_at"
        )
        .bind(id)
        .bind(name)
        .bind(email)
        .bind(company)
        .bind(title)
        .bind(password_hash)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(developer)
    }

    pub async fn find_developer_by_email(&self, email: &str) -> AppResult<Option<Developer>> {
        let developer = sqlx::query_as::<_, Developer>(
            "SELECT id, name, email, company, title, password_hash, created_at, updated_at FROM developers WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(developer)
    }

    pub async fn find_developer_by_id(&self, id: Uuid) -> AppResult<Option<Developer>> {
        let developer = sqlx::query_as::<_, Developer>(
            "SELECT id, name, email, password_hash, created_at, updated_at FROM developers WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(developer)
    }

    pub async fn create_project(
        &self,
        developer_id: Uuid,
        name: &str,
        description: &str,
        environment: ProjectEnvironment,
        client_id: &str,
        client_secret_hash: &str,
        redirect_uris: &[String],
        scopes: &[String],
    ) -> AppResult<Project> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO projects (id, developer_id, name, description, environment, client_id, client_secret_hash, redirect_uris, scopes, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id, developer_id, name, description, environment, client_id, client_secret_hash, redirect_uris, scopes, is_active, created_at, updated_at"
        )
        .bind(id)
        .bind(developer_id)
        .bind(name)
        .bind(description)
        .bind(environment)
        .bind(client_id)
        .bind(client_secret_hash)
        .bind(redirect_uris)
        .bind(scopes)
        .bind(true)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(project)
    }

    pub async fn find_project_by_client_id(&self, client_id: &str) -> AppResult<Option<Project>> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT id, developer_id, name, description, environment, client_id, client_secret_hash, redirect_uris, scopes, is_active, created_at, updated_at FROM projects WHERE client_id = $1"
        )
        .bind(client_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(project)
    }

    pub async fn store_oauth_token(&self, token: &OAuthToken) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO oauth_tokens (id, project_id, developer_id, access_token_hash, token_type, scopes, expires_at, jti, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(token.id)
        .bind(token.project_id)
        .bind(token.developer_id)
        .bind(&token.access_token_hash)
        .bind(&token.token_type)
        .bind(&token.scopes)
        .bind(token.expires_at)
        .bind(&token.jti)
        .bind(token.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(())
    }

    pub async fn find_oauth_token_by_jti(&self, jti: &str) -> AppResult<Option<OAuthToken>> {
        let token = sqlx::query_as::<_, OAuthToken>(
            "SELECT id, project_id, developer_id, access_token_hash, token_type, scopes, expires_at, jti, created_at FROM oauth_tokens WHERE jti = $1"
        )
        .bind(jti)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(token)
    }

    pub async fn get_oauth_token_by_jti(&self, jti: &str) -> AppResult<Option<OAuthToken>> {
        self.find_oauth_token_by_jti(jti).await
    }

    pub async fn revoke_oauth_token(&self, jti: &str) -> AppResult<()> {
        sqlx::query("DELETE FROM oauth_tokens WHERE jti = $1")
            .bind(jti)
            .execute(&self.pool)
            .await
            .map_err(|e| crate::core::error::AppError::Database(e))?;

        Ok(())
    }
}

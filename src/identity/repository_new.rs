use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::identity::model::{
    FaceEmbedding, FraudAlert, FraudRiskLevel, IdentityError, IdentityResult, IdentityVerification,
    VerificationStatus,
};

/// Repository for identity-related database operations
#[derive(Clone)]
pub struct IdentityRepository {
    pool: PgPool,
}

impl IdentityRepository {
    /// Create a new identity repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Initialize database tables and extensions
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing identity database schema...");

        // Enable pgvector extension
        sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
            .execute(&self.pool)
            .await?;

        // Create face_embeddings table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS face_embeddings (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL,
                embedding VECTOR(512) NOT NULL,
                model_version VARCHAR(50) NOT NULL,
                quality_score REAL NOT NULL,
                enrollment_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                
                CONSTRAINT face_embeddings_user_id_fkey 
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create identity_verifications table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS identity_verifications (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID NOT NULL,
                verification_type VARCHAR(50) NOT NULL,
                biometric_type VARCHAR(50) NOT NULL,
                status VARCHAR(20) NOT NULL,
                confidence_score REAL NOT NULL,
                face_match_score REAL,
                liveness_score REAL,
                fraud_risk_score REAL,
                document_type VARCHAR(50),
                document_number VARCHAR(100),
                verification_data JSONB,
                provider VARCHAR(50),
                provider_reference VARCHAR(100),
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                completed_at TIMESTAMPTZ,
                
                CONSTRAINT identity_verifications_user_id_fkey 
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create fraud_alerts table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS fraud_alerts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                user_id UUID,
                duplicate_user_id UUID,
                alert_type VARCHAR(100) NOT NULL,
                similarity_score REAL NOT NULL,
                confidence_score REAL NOT NULL,
                status VARCHAR(20) NOT NULL DEFAULT 'open',
                investigated_by UUID,
                investigation_notes TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                
                CONSTRAINT fraud_alerts_user_id_fkey 
                    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for performance
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_face_embeddings_user_id ON face_embeddings(user_id)",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_face_embeddings_active ON face_embeddings(is_active) WHERE is_active = TRUE")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_identity_verifications_user_id ON identity_verifications(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_identity_verifications_status ON identity_verifications(status)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_fraud_alerts_user_id ON fraud_alerts(user_id)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_fraud_alerts_status ON fraud_alerts(status)")
            .execute(&self.pool)
            .await?;

        info!("Identity database schema initialized successfully");
        Ok(())
    }

    // Face Embedding Operations

    /// Store a face embedding for a user
    pub async fn store_face_embedding(&self, embedding: &FaceEmbedding) -> IdentityResult<Uuid> {
        let embedding_vec: Vec<f32> = embedding.embedding.clone();

        let row = sqlx::query(
            r#"
            INSERT INTO face_embeddings (user_id, embedding, model_version, quality_score)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#,
        )
        .bind(&embedding.user_id)
        .bind(&embedding_vec)
        .bind(&embedding.model_version)
        .bind(embedding.quality_score)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        let id: Uuid = row.get("id");
        info!(
            "Stored face embedding for user {}: {}",
            embedding.user_id, id
        );

        Ok(id)
    }

    /// Find similar face embeddings using vector similarity
    pub async fn find_similar_embeddings(
        &self,
        embedding: &[f32],
        threshold: f32,
        limit: i64,
    ) -> IdentityResult<Vec<FaceEmbedding>> {
        let rows = sqlx::query(
            "SELECT id, user_id, embedding, model_version, quality_score, 
             enrollment_date, created_at, updated_at, is_active,
             (embedding <=> $1::vector) as distance
             FROM face_embeddings 
             WHERE is_active = true 
             AND (embedding <=> $1::vector) < $2
             ORDER BY embedding <=> $1::vector 
             LIMIT $3",
        )
        .bind(embedding)
        .bind(threshold)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        let mut embeddings = Vec::new();
        for row in rows {
            let embedding_vec: Vec<f32> = row.get("embedding");
            embeddings.push(FaceEmbedding {
                id: row.get("id"),
                user_id: row.get("user_id"),
                embedding: embedding_vec,
                model_version: row.get("model_version"),
                quality_score: row.get("quality_score"),
                enrollment_date: row.get("enrollment_date"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                is_active: row.get("is_active"),
            });
        }

        Ok(embeddings)
    }

    /// Get face embeddings for a specific user
    pub async fn get_user_embeddings(&self, user_id: Uuid) -> IdentityResult<Vec<FaceEmbedding>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, embedding, model_version, quality_score, 
                   enrollment_date, created_at, updated_at, is_active
            FROM face_embeddings 
            WHERE user_id = $1 AND is_active = TRUE
            ORDER BY created_at DESC
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        let mut embeddings = Vec::new();
        for row in rows {
            let embedding_vec: Vec<f32> = row.get("embedding");
            embeddings.push(FaceEmbedding {
                id: row.get("id"),
                user_id: row.get("user_id"),
                embedding: embedding_vec,
                model_version: row.get("model_version"),
                quality_score: row.get("quality_score"),
                enrollment_date: row.get("enrollment_date"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                is_active: row.get("is_active"),
            });
        }

        Ok(embeddings)
    }

    // Identity Verification Operations

    /// Store an identity verification record
    pub async fn store_verification(
        &self,
        verification: &IdentityVerification,
    ) -> IdentityResult<Uuid> {
        let row = sqlx::query(
            r#"
            INSERT INTO identity_verifications (
                user_id, verification_type, biometric_type, status, confidence_score, 
                liveness_score, fraud_risk_score, document_type, 
                document_number, verification_data, provider, provider_reference
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
        "#,
        )
        .bind(&verification.user_id)
        .bind(&verification.verification_type)
        .bind(&verification.biometric_type)
        .bind(&verification.status.to_string())
        .bind(verification.confidence_score)
        .bind(verification.liveness_score)
        .bind(verification.fraud_risk_score)
        .bind(&verification.document_type)
        .bind(&verification.document_number)
        .bind(&verification.verification_data)
        .bind(&verification.provider)
        .bind(&verification.provider_reference)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        let id: Uuid = row.get("id");
        info!(
            "Stored identity verification for user {}: {}",
            verification.user_id, id
        );

        Ok(id)
    }

    /// Get latest verification for a user
    pub async fn get_latest_verification(
        &self,
        user_id: Uuid,
    ) -> IdentityResult<Option<IdentityVerification>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, verification_type, biometric_type, status, confidence_score, 
                   face_match_score, liveness_score, fraud_risk_score, document_type, 
                   document_number, verification_data, provider, provider_reference,
                   created_at, updated_at, completed_at
            FROM identity_verifications 
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
        "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            let status_str: String = row.get("status");
            let status = VerificationStatus::from_str(&status_str)
                .map_err(|_| IdentityError::Internal("Invalid verification status".to_string()))?;

            Ok(Some(IdentityVerification {
                id: row.get("id"),
                user_id: row.get("user_id"),
                verification_type: row.get("verification_type"),
                biometric_type: row.get("biometric_type"),
                status,
                confidence_score: row.get("confidence_score"),
                liveness_score: row.get("liveness_score"),
                fraud_risk_score: row.get("fraud_risk_score"),
                document_type: row.get("document_type"),
                document_number: row.get("document_number"),
                verification_data: row.get("verification_data"),
                provider: row.get("provider"),
                provider_reference: row.get("provider_reference"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                completed_at: row.get("completed_at"),
            }))
        } else {
            Ok(None)
        }
    }

    // Fraud Alert Operations

    /// Store a fraud alert
    pub async fn store_fraud_alert(&self, alert: &FraudAlert) -> IdentityResult<Uuid> {
        let row = sqlx::query(
            r#"
            INSERT INTO fraud_alerts (
                user_id, duplicate_user_id, alert_type, similarity_score, 
                status, investigated_by, investigation_notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
        "#,
        )
        .bind(alert.user_id)
        .bind(alert.duplicate_user_id)
        .bind(&alert.alert_type)
        .bind(alert.similarity_score)
        .bind(&alert.status)
        .bind(alert.investigated_by)
        .bind(&alert.investigation_notes)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        let id: Uuid = row.get("id");
        warn!(
            "Stored fraud alert for user {:?}: {} ({})",
            alert.user_id, id, alert.alert_type
        );

        Ok(id)
    }

    /// Get fraud alerts for a user
    pub async fn get_user_fraud_alerts(&self, user_id: Uuid) -> IdentityResult<Vec<FraudAlert>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, duplicate_user_id, alert_type, similarity_score, 
                   confidence_score, status, investigated_by, investigation_notes, 
                   created_at, updated_at
            FROM fraud_alerts 
            WHERE user_id = $1
            ORDER BY created_at DESC
        "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        let mut alerts = Vec::new();
        for row in rows {
            let status_str: String = row.get("status");
            let status = FraudStatus::from_str(&status_str)
                .map_err(|_| IdentityError::Internal("Invalid fraud status".to_string()))?;

            alerts.push(FraudAlert {
                id: row.get("id"),
                user_id: row.get("user_id"),
                duplicate_user_id: row.get("duplicate_user_id"),
                alert_type: row.get("alert_type"),
                similarity_score: row.get("similarity_score"),
                status: status.to_string(),
                investigated_by: row.get("investigated_by"),
                investigation_notes: row.get("investigation_notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(alerts)
    }

    // Statistics and Analytics

    /// Get verification statistics for a user
    pub async fn get_user_verification_stats(
        &self,
        user_id: Uuid,
    ) -> IdentityResult<VerificationStats> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_verifications,
                COUNT(*) FILTER (WHERE status = 'verified') as successful_verifications,
                COUNT(*) FILTER (WHERE status = 'failed') as failed_verifications,
                AVG(confidence_score) as avg_confidence_score,
                MAX(created_at) as last_verification_at
            FROM identity_verifications 
            WHERE user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| IdentityError::DatabaseError(e.to_string()))?;

        Ok(VerificationStats {
            total_verifications: row.get::<i64, _>("total_verifications") as u32,
            successful_verifications: row.get::<i64, _>("successful_verifications") as u32,
            failed_verifications: row.get::<i64, _>("failed_verifications") as u32,
            avg_confidence_score: row.get("avg_confidence_score"),
            last_verification_at: row.get("last_verification_at"),
        })
    }
}

// Helper structs and enums

#[derive(Debug, Clone)]
pub enum FraudStatus {
    Open,
    Investigating,
    Resolved,
    FalsePositive,
}

impl FraudStatus {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "open" => Ok(FraudStatus::Open),
            "investigating" => Ok(FraudStatus::Investigating),
            "resolved" => Ok(FraudStatus::Resolved),
            "false_positive" => Ok(FraudStatus::FalsePositive),
            _ => Err("Invalid fraud status"),
        }
    }

    fn to_string(&self) -> String {
        match self {
            FraudStatus::Open => "open".to_string(),
            FraudStatus::Investigating => "investigating".to_string(),
            FraudStatus::Resolved => "resolved".to_string(),
            FraudStatus::FalsePositive => "false_positive".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct VerificationStats {
    pub total_verifications: u32,
    pub successful_verifications: u32,
    pub failed_verifications: u32,
    pub avg_confidence_score: Option<f64>,
    pub last_verification_at: Option<DateTime<Utc>>,
}

// String conversion traits
trait FromStr: Sized {
    fn from_str(s: &str) -> Result<Self, &'static str>;
}

impl FromStr for VerificationStatus {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "pending" => Ok(VerificationStatus::Pending),
            "verified" => Ok(VerificationStatus::Verified),
            "failed" => Ok(VerificationStatus::Failed),
            "expired" => Ok(VerificationStatus::Expired),
            "flagged" => Ok(VerificationStatus::Flagged),
            _ => Err("Invalid verification status"),
        }
    }
}

impl ToString for VerificationStatus {
    fn to_string(&self) -> String {
        match self {
            VerificationStatus::Pending => "pending".to_string(),
            VerificationStatus::InProgress => "in_progress".to_string(),
            VerificationStatus::Completed => "completed".to_string(),
            VerificationStatus::Verified => "verified".to_string(),
            VerificationStatus::Failed => "failed".to_string(),
            VerificationStatus::Expired => "expired".to_string(),
            VerificationStatus::Flagged => "flagged".to_string(),
        }
    }
}

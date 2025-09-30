-- Add pgvector extension for vector similarity search
CREATE EXTENSION IF NOT EXISTS vector;

-- Face embeddings table for storing biometric face data
CREATE TABLE IF NOT EXISTS face_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    embedding VECTOR(512) NOT NULL,
    model_version VARCHAR(50) NOT NULL,
    quality_score REAL NOT NULL CHECK (quality_score >= 0 AND quality_score <= 1),
    enrollment_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    
    CONSTRAINT face_embeddings_user_id_fkey 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Identity verifications table for tracking all verification attempts
CREATE TABLE IF NOT EXISTS identity_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    verification_type VARCHAR(50) NOT NULL,
    biometric_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'verified', 'failed', 'expired', 'flagged')),
    confidence_score REAL NOT NULL CHECK (confidence_score >= 0 AND confidence_score <= 1),
    face_match_score REAL CHECK (face_match_score >= 0 AND face_match_score <= 1),
    liveness_score REAL CHECK (liveness_score >= 0 AND liveness_score <= 1),
    fraud_risk_score REAL CHECK (fraud_risk_score >= 0 AND fraud_risk_score <= 1),
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
);

-- Fraud alerts table for tracking suspicious activities
CREATE TABLE IF NOT EXISTS fraud_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    duplicate_user_id UUID,
    alert_type VARCHAR(100) NOT NULL,
    similarity_score REAL NOT NULL CHECK (similarity_score >= 0 AND similarity_score <= 1),
    confidence_score REAL NOT NULL CHECK (confidence_score >= 0 AND confidence_score <= 1),
    status VARCHAR(20) NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'investigating', 'resolved', 'false_positive')),
    investigated_by UUID,
    investigation_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT fraud_alerts_user_id_fkey 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fraud_alerts_duplicate_user_id_fkey 
        FOREIGN KEY (duplicate_user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fraud_alerts_investigated_by_fkey 
        FOREIGN KEY (investigated_by) REFERENCES users(id) ON DELETE SET NULL
);

-- Verification sessions table for multi-step verification workflows
CREATE TABLE IF NOT EXISTS verification_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    session_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'completed', 'expired', 'cancelled')),
    steps_completed JSONB NOT NULL DEFAULT '[]',
    steps_required JSONB NOT NULL,
    current_step VARCHAR(50),
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    
    CONSTRAINT verification_sessions_user_id_fkey 
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Performance indexes
CREATE INDEX IF NOT EXISTS idx_face_embeddings_user_id ON face_embeddings(user_id);
CREATE INDEX IF NOT EXISTS idx_face_embeddings_active ON face_embeddings(is_active) WHERE is_active = TRUE;
CREATE INDEX IF NOT EXISTS idx_face_embeddings_quality ON face_embeddings(quality_score DESC);
CREATE INDEX IF NOT EXISTS idx_face_embeddings_model_version ON face_embeddings(model_version);

-- Vector similarity index for fast face matching
CREATE INDEX IF NOT EXISTS idx_face_embeddings_embedding ON face_embeddings 
USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);

CREATE INDEX IF NOT EXISTS idx_identity_verifications_user_id ON identity_verifications(user_id);
CREATE INDEX IF NOT EXISTS idx_identity_verifications_status ON identity_verifications(status);
CREATE INDEX IF NOT EXISTS idx_identity_verifications_type ON identity_verifications(verification_type);
CREATE INDEX IF NOT EXISTS idx_identity_verifications_created_at ON identity_verifications(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_identity_verifications_confidence ON identity_verifications(confidence_score DESC);

CREATE INDEX IF NOT EXISTS idx_fraud_alerts_user_id ON fraud_alerts(user_id);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_status ON fraud_alerts(status);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_type ON fraud_alerts(alert_type);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_similarity ON fraud_alerts(similarity_score DESC);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_created_at ON fraud_alerts(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_verification_sessions_user_id ON verification_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_verification_sessions_status ON verification_sessions(status);
CREATE INDEX IF NOT EXISTS idx_verification_sessions_expires_at ON verification_sessions(expires_at);

-- Composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_face_embeddings_user_active ON face_embeddings(user_id, is_active) WHERE is_active = TRUE;
CREATE INDEX IF NOT EXISTS idx_verifications_user_status ON identity_verifications(user_id, status);
CREATE INDEX IF NOT EXISTS idx_fraud_alerts_user_status ON fraud_alerts(user_id, status);

-- Add triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_face_embeddings_updated_at BEFORE UPDATE ON face_embeddings 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_identity_verifications_updated_at BEFORE UPDATE ON identity_verifications 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_fraud_alerts_updated_at BEFORE UPDATE ON fraud_alerts 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_verification_sessions_updated_at BEFORE UPDATE ON verification_sessions 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Add comments for documentation
COMMENT ON TABLE face_embeddings IS 'Stores face embeddings for biometric identification using pgvector for similarity search';
COMMENT ON TABLE identity_verifications IS 'Tracks all identity verification attempts with ML confidence scores and fraud indicators';
COMMENT ON TABLE fraud_alerts IS 'Records suspicious activities and potential fraud attempts during identity verification';
COMMENT ON TABLE verification_sessions IS 'Manages multi-step verification workflows and session state';

COMMENT ON COLUMN face_embeddings.embedding IS 'Face embedding vector (512 dimensions) for similarity matching';
COMMENT ON COLUMN face_embeddings.quality_score IS 'Image quality score from 0.0 to 1.0';
COMMENT ON COLUMN identity_verifications.confidence_score IS 'Overall verification confidence from 0.0 to 1.0';
COMMENT ON COLUMN fraud_alerts.similarity_score IS 'Face similarity score that triggered the fraud alert';
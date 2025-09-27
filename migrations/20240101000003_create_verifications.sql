-- Create verification status enum
CREATE TYPE verification_status AS ENUM ('pending', 'in_progress', 'completed', 'failed', 'expired');

-- Create identity_verifications table
CREATE TABLE IF NOT EXISTS identity_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    verification_type VARCHAR(100) NOT NULL,
    status verification_status DEFAULT 'pending',
    document_type VARCHAR(100),
    document_number VARCHAR(255),
    verification_data JSONB,
    provider VARCHAR(100),
    provider_reference VARCHAR(255),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create income verification status enum
CREATE TYPE income_verification_status AS ENUM ('pending', 'in_progress', 'completed', 'failed', 'expired');

-- Create income_verifications table
CREATE TABLE IF NOT EXISTS income_verifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    verification_type VARCHAR(100) NOT NULL,
    status income_verification_status DEFAULT 'pending',
    employer_name VARCHAR(255),
    job_title VARCHAR(255),
    annual_income BIGINT,
    currency VARCHAR(3) DEFAULT 'USD',
    verification_data JSONB,
    provider VARCHAR(100),
    provider_reference VARCHAR(255),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_identity_verifications_user_id ON identity_verifications(user_id);
CREATE INDEX IF NOT EXISTS idx_identity_verifications_status ON identity_verifications(status);
CREATE INDEX IF NOT EXISTS idx_income_verifications_user_id ON income_verifications(user_id);
CREATE INDEX IF NOT EXISTS idx_income_verifications_status ON income_verifications(status);
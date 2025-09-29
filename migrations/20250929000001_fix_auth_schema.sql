-- Migration: Fix Auth Schema to Match Current Code
-- This aligns the database schema with the current OAuth2 implementation
-- Date: 2025-09-29

-- Fix developers table to match current Developer model
ALTER TABLE developers 
ADD COLUMN IF NOT EXISTS name VARCHAR(255);

-- Update existing developers to have name from first_name + last_name if name is null
UPDATE developers 
SET name = CONCAT(first_name, ' ', last_name) 
WHERE name IS NULL;

-- Make name NOT NULL after populating it
ALTER TABLE developers 
ALTER COLUMN name SET NOT NULL;

-- Fix projects table to match current Project model
ALTER TABLE projects 
ADD COLUMN IF NOT EXISTS redirect_uris TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS scopes TEXT[] DEFAULT '{}';

-- Update projects table environment enum to match ProjectEnvironment
ALTER TABLE projects 
DROP CONSTRAINT IF EXISTS projects_environment_check;

ALTER TABLE projects 
ADD CONSTRAINT projects_environment_check 
CHECK (environment IN ('development', 'staging', 'production'));

-- Update existing sandbox to development
UPDATE projects 
SET environment = 'development' 
WHERE environment = 'sandbox';

-- Fix oauth_tokens table to match current OAuthToken model
DROP TABLE IF EXISTS oauth_tokens CASCADE;

CREATE TABLE oauth_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    developer_id UUID NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
    access_token_hash VARCHAR(255) NOT NULL,
    token_type VARCHAR(50) NOT NULL DEFAULT 'Bearer',
    scopes TEXT[] NOT NULL DEFAULT '{}',
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    jti VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add indexes for oauth_tokens
CREATE INDEX idx_oauth_tokens_project_id ON oauth_tokens(project_id);
CREATE INDEX idx_oauth_tokens_developer_id ON oauth_tokens(developer_id);
CREATE INDEX idx_oauth_tokens_jti ON oauth_tokens(jti);
CREATE INDEX idx_oauth_tokens_expires_at ON oauth_tokens(expires_at);

-- Add some sample data for testing (optional)
-- This creates a test developer and project for immediate testing
DO $$
DECLARE
    test_dev_id UUID;
    test_project_id UUID;
BEGIN
    -- Insert test developer if it doesn't exist
    INSERT INTO developers (email, password_hash, name, first_name, last_name, company_name, is_verified, is_active)
    VALUES (
        'test@openbank.dev',
        '$2b$12$LQv3c1yqBwEHFxxkdsHzAOBNNipNiS2F/F7SFhd1wjLUHMTAq1N8C', -- password: testpass123
        'Test Developer',
        'Test',
        'Developer', 
        'OpenBank Test Corp',
        true,
        true
    )
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO test_dev_id;
    
    -- Get the developer ID if it already exists
    IF test_dev_id IS NULL THEN
        SELECT id INTO test_dev_id FROM developers WHERE email = 'test@openbank.dev';
    END IF;
    
    -- Insert test project if developer exists
    IF test_dev_id IS NOT NULL THEN
        INSERT INTO projects (
            developer_id, 
            name, 
            description, 
            client_id, 
            client_secret_hash, 
            environment,
            redirect_uris,
            scopes,
            is_active
        )
        VALUES (
            test_dev_id,
            'Test Banking API',
            'Test project for OpenBank API development',
            'ck_test_client_id_12345',
            '$2b$12$test_client_secret_hash_placeholder', 
            'development',
            ARRAY['http://localhost:3000/callback', 'http://localhost:8080/callback'],
            ARRAY['read:accounts', 'write:transactions', 'read:balance'],
            true
        )
        ON CONFLICT (client_id) DO NOTHING;
    END IF;
END $$;

-- Verify schema is correct
DO $$
BEGIN
    -- Check if all required columns exist
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'developers' AND column_name = 'name'
    ) THEN
        RAISE EXCEPTION 'Migration failed: developers.name column missing';
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'projects' AND column_name = 'redirect_uris'
    ) THEN
        RAISE EXCEPTION 'Migration failed: projects.redirect_uris column missing';
    END IF;
    
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'oauth_tokens' AND column_name = 'jti'
    ) THEN
        RAISE EXCEPTION 'Migration failed: oauth_tokens.jti column missing';
    END IF;
    
    RAISE NOTICE 'Auth schema migration completed successfully!';
END $$;
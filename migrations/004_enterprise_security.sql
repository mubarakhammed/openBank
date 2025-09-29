-- Enterprise Security Schema Migration
-- Add security tables for account protection, RBAC, and audit compliance

-- Account Security Table
CREATE TABLE IF NOT EXISTS account_security (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    developer_id UUID NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
    
    -- Failed login tracking
    failed_attempts INTEGER DEFAULT 0,
    last_failed_attempt TIMESTAMP WITH TIME ZONE,
    
    -- Account lockout
    locked_until TIMESTAMP WITH TIME ZONE,
    lock_reason TEXT,
    
    -- Login tracking
    last_successful_login TIMESTAMP WITH TIME ZONE,
    login_count BIGINT DEFAULT 0,
    
    -- Suspicious activity tracking
    suspicious_activity_score INTEGER DEFAULT 0,
    suspicious_ips TEXT[] DEFAULT ARRAY[]::TEXT[],
    
    -- Password security
    password_last_changed TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    password_history_hashes TEXT[] DEFAULT ARRAY[]::TEXT[],
    
    -- MFA settings (future implementation)
    mfa_enabled BOOLEAN DEFAULT FALSE,
    mfa_secret TEXT,
    backup_codes TEXT[] DEFAULT ARRAY[]::TEXT[],
    
    -- Security preferences
    security_notifications BOOLEAN DEFAULT TRUE,
    login_alerts BOOLEAN DEFAULT TRUE,
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(developer_id)
);

-- User Roles Table (RBAC)
CREATE TABLE IF NOT EXISTS user_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    developer_id UUID NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('super_admin', 'admin', 'developer', 'read_only', 'support', 'auditor')),
    
    -- Role management
    granted_by UUID REFERENCES developers(id),
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    
    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(developer_id, role)
);

-- Custom User Permissions Table
CREATE TABLE IF NOT EXISTS user_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    developer_id UUID NOT NULL REFERENCES developers(id) ON DELETE CASCADE,
    
    -- Permission definition
    resource TEXT NOT NULL,
    action TEXT NOT NULL,
    conditions JSONB,
    
    -- Permission management
    granted_by UUID REFERENCES developers(id),
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    
    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    -- Constraints
    UNIQUE(developer_id, resource, action)
);

-- Rate Limiting Table (if not using Redis)
CREATE TABLE IF NOT EXISTS rate_limits (
    ip_address INET PRIMARY KEY,
    request_count INTEGER DEFAULT 0,
    window_start TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    blocked_until TIMESTAMP WITH TIME ZONE,
    violation_count INTEGER DEFAULT 0,
    
    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Security Events Table (backup for MongoDB)
CREATE TABLE IF NOT EXISTS security_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('info', 'warning', 'error', 'critical')),
    
    -- Context
    developer_id UUID REFERENCES developers(id),
    project_id UUID REFERENCES projects(id),
    ip_address INET,
    user_agent TEXT,
    
    -- Event details
    success BOOLEAN DEFAULT TRUE,
    error_message TEXT,
    metadata JSONB,
    risk_score INTEGER CHECK (risk_score >= 0 AND risk_score <= 100),
    
    -- Compliance
    compliance_tags TEXT[] DEFAULT ARRAY[]::TEXT[],
    
    -- Timestamps
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Indexes for Performance
CREATE INDEX IF NOT EXISTS idx_account_security_developer_id ON account_security(developer_id);
CREATE INDEX IF NOT EXISTS idx_account_security_locked_until ON account_security(locked_until) WHERE locked_until IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_account_security_failed_attempts ON account_security(failed_attempts) WHERE failed_attempts > 0;

CREATE INDEX IF NOT EXISTS idx_user_roles_developer_id ON user_roles(developer_id);
CREATE INDEX IF NOT EXISTS idx_user_roles_role ON user_roles(role);
CREATE INDEX IF NOT EXISTS idx_user_roles_expires_at ON user_roles(expires_at) WHERE expires_at IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_user_permissions_developer_id ON user_permissions(developer_id);
CREATE INDEX IF NOT EXISTS idx_user_permissions_resource ON user_permissions(resource);
CREATE INDEX IF NOT EXISTS idx_user_permissions_expires_at ON user_permissions(expires_at) WHERE expires_at IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_rate_limits_window ON rate_limits(window_start);
CREATE INDEX IF NOT EXISTS idx_rate_limits_blocked ON rate_limits(blocked_until) WHERE blocked_until IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_security_events_developer_id ON security_events(developer_id);
CREATE INDEX IF NOT EXISTS idx_security_events_timestamp ON security_events(timestamp);
CREATE INDEX IF NOT EXISTS idx_security_events_event_type ON security_events(event_type);
CREATE INDEX IF NOT EXISTS idx_security_events_severity ON security_events(severity);
CREATE INDEX IF NOT EXISTS idx_security_events_compliance ON security_events USING GIN(compliance_tags);

-- Triggers for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_account_security_updated_at BEFORE UPDATE ON account_security FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_rate_limits_updated_at BEFORE UPDATE ON rate_limits FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Default roles for existing developers
INSERT INTO user_roles (developer_id, role)
SELECT id, 'developer' FROM developers 
WHERE id NOT IN (SELECT developer_id FROM user_roles);

-- Default account security for existing developers
INSERT INTO account_security (developer_id)
SELECT id FROM developers 
WHERE id NOT IN (SELECT developer_id FROM account_security);

-- Grant statement
COMMENT ON TABLE account_security IS 'Enterprise account security tracking and protection';
COMMENT ON TABLE user_roles IS 'Role-based access control (RBAC) system';
COMMENT ON TABLE user_permissions IS 'Custom permission grants for fine-grained access control';
COMMENT ON TABLE rate_limits IS 'Rate limiting state (alternative to Redis)';
COMMENT ON TABLE security_events IS 'Security event logging (backup to MongoDB)';
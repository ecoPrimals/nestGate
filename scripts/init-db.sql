-- NestGate Database Initialization Script
-- This script sets up the initial database schema for NestGate

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gin";

-- Create schemas
CREATE SCHEMA IF NOT EXISTS nestgate;
CREATE SCHEMA IF NOT EXISTS monitoring;
CREATE SCHEMA IF NOT EXISTS security;

-- Set search path
SET search_path TO nestgate, public;

-- Users and authentication
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE
);

-- Services registry
CREATE TABLE IF NOT EXISTS services (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    endpoint TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    capabilities JSONB,
    metadata JSONB,
    health_check_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(name, version)
);

-- Storage systems
CREATE TABLE IF NOT EXISTS storage_systems (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) UNIQUE NOT NULL,
    type VARCHAR(100) NOT NULL, -- 'zfs', 'filesystem', 'object', etc.
    configuration JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    capacity_bytes BIGINT,
    used_bytes BIGINT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Storage pools (ZFS specific)
CREATE TABLE IF NOT EXISTS storage_pools (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    system_id UUID REFERENCES storage_systems(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    health VARCHAR(50) NOT NULL DEFAULT 'ONLINE',
    capacity_bytes BIGINT NOT NULL,
    used_bytes BIGINT NOT NULL DEFAULT 0,
    free_bytes BIGINT NOT NULL,
    properties JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(system_id, name)
);

-- Storage datasets
CREATE TABLE IF NOT EXISTS storage_datasets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pool_id UUID REFERENCES storage_pools(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL DEFAULT 'filesystem',
    mountpoint TEXT,
    used_bytes BIGINT NOT NULL DEFAULT 0,
    available_bytes BIGINT NOT NULL DEFAULT 0,
    properties JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(pool_id, name)
);

-- Configuration store
CREATE TABLE IF NOT EXISTS configurations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(255) UNIQUE NOT NULL,
    value JSONB NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Monitoring schema
SET search_path TO monitoring, public;

-- Performance metrics
CREATE TABLE IF NOT EXISTS metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    service_name VARCHAR(255) NOT NULL,
    metric_name VARCHAR(255) NOT NULL,
    metric_value DOUBLE PRECISION NOT NULL,
    labels JSONB,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index for time-series queries
CREATE INDEX IF NOT EXISTS idx_metrics_service_time ON metrics(service_name, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_name_time ON metrics(metric_name, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_labels ON metrics USING GIN(labels);

-- Health checks
CREATE TABLE IF NOT EXISTS health_checks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    service_id UUID,
    check_name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    message TEXT,
    response_time_ms INTEGER,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Security schema
SET search_path TO security, public;

-- Audit logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID,
    action VARCHAR(255) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- API tokens
CREATE TABLE IF NOT EXISTS api_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES nestgate.users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    token_hash TEXT UNIQUE NOT NULL,
    permissions JSONB,
    expires_at TIMESTAMP WITH TIME ZONE,
    last_used TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Security events
CREATE TABLE IF NOT EXISTS security_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_type VARCHAR(100) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'info',
    source_ip INET,
    user_id UUID,
    details JSONB,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_time ON security.audit_logs(user_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action_time ON security.audit_logs(action, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_security_events_type_time ON security.security_events(event_type, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_security_events_severity_time ON security.security_events(severity, timestamp DESC);

-- Reset search path
SET search_path TO public;

-- Insert default configurations
INSERT INTO nestgate.configurations (key, value, description) VALUES 
('system.version', '"2.0.0"', 'NestGate system version'),
('api.rate_limit.requests_per_minute', '1000', 'API rate limit per minute'),
('storage.default_pool', '"tank"', 'Default storage pool name'),
('security.session_timeout_minutes', '30', 'User session timeout in minutes'),
('monitoring.metrics_retention_days', '30', 'Metrics retention period')
ON CONFLICT (key) DO NOTHING;

-- Create default admin user (password: admin123)
-- Note: In production, change this password immediately
INSERT INTO nestgate.users (username, email, password_hash, role) VALUES 
('admin', 'admin@nestgate.local', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewKyNiTGk5KGIlSi', 'admin')
ON CONFLICT (username) DO NOTHING;

-- Grant permissions
GRANT USAGE ON SCHEMA nestgate TO nestgate;
GRANT USAGE ON SCHEMA monitoring TO nestgate;
GRANT USAGE ON SCHEMA security TO nestgate;

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA nestgate TO nestgate;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA monitoring TO nestgate;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA security TO nestgate;

GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA nestgate TO nestgate;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA monitoring TO nestgate;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA security TO nestgate; 
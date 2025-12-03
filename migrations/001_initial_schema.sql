-- Migration: Create ephemeral vault schema

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY,
    user_wallet VARCHAR(255) NOT NULL,
    ephemeral_wallet VARCHAR(255) NOT NULL,
    ephemeral_keypair BYTEA NOT NULL,
    vault_address VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    total_deposits BIGINT NOT NULL DEFAULT 0,
    total_spent BIGINT NOT NULL DEFAULT 0,
    ip_address VARCHAR(45) NOT NULL,
    device_fingerprint VARCHAR(64),
    created_at_idx TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sessions_user_wallet ON sessions(user_wallet);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_is_active ON sessions(is_active);

-- Vault transactions table
CREATE TABLE IF NOT EXISTS vault_transactions (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    transaction_type VARCHAR(50) NOT NULL,
    amount BIGINT NOT NULL,
    fee BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    status VARCHAR(20) NOT NULL,
    signature VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_vault_transactions_session_id ON vault_transactions(session_id);
CREATE INDEX idx_vault_transactions_timestamp ON vault_transactions(timestamp);

-- Delegation records table
CREATE TABLE IF NOT EXISTS delegations (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    vault_address VARCHAR(255) NOT NULL,
    delegated_to VARCHAR(255) NOT NULL,
    approved_at TIMESTAMP WITH TIME ZONE NOT NULL,
    revoked_at TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_delegations_session_id ON delegations(session_id);
CREATE INDEX idx_delegations_vault_address ON delegations(vault_address);

-- Cleanup events table
CREATE TABLE IF NOT EXISTS cleanup_events (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    vault_address VARCHAR(255) NOT NULL,
    returned_amount BIGINT NOT NULL,
    cleaned_at TIMESTAMP WITH TIME ZONE NOT NULL,
    reason VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cleanup_events_session_id ON cleanup_events(session_id);
CREATE INDEX idx_cleanup_events_cleaned_at ON cleanup_events(cleaned_at);

-- Security events table
CREATE TABLE IF NOT EXISTS security_events (
    id UUID PRIMARY KEY,
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    event_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    description TEXT NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_security_events_session_id ON security_events(session_id);
CREATE INDEX idx_security_events_timestamp ON security_events(timestamp);
CREATE INDEX idx_security_events_severity ON security_events(severity);

-- User analytics view
CREATE VIEW user_session_analytics AS
SELECT
    s.user_wallet,
    COUNT(DISTINCT s.id) as total_sessions,
    COUNT(CASE WHEN s.is_active = true AND s.expires_at > NOW() THEN 1 END) as active_sessions,
    COALESCE(SUM(s.total_deposits), 0) as total_funds_processed,
    COALESCE(AVG(EXTRACT(EPOCH FROM (s.expires_at - s.created_at))), 0) as avg_session_duration_secs,
    MAX(s.created_at) as last_session_start
FROM sessions s
GROUP BY s.user_wallet;

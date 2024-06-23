-- Add up migration script here
CREATE TABLE IF NOT EXISTS accounts (
    id SERIAL PRIMARY KEY,
    ownership_id UUID UNIQUE NOT NULL,
    trace_id UUID UNIQUE NOT NULL,
    public_id UUID UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    document_registry VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    disabled_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_accounts_ownership_id ON accounts (ownership_id);
CREATE INDEX IF NOT EXISTS idx_accounts_trace_id ON accounts (trace_id);
CREATE INDEX IF NOT EXISTS idx_accounts_public_id ON accounts (public_id);
CREATE INDEX IF NOT EXISTS idx_accounts_deleted_at ON accounts (deleted_at);

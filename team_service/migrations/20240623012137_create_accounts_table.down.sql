-- Add down migration script here
-- Add down migration script here
DROP INDEX IF EXISTS idx_accounts_deleted_at;
DROP INDEX IF EXISTS idx_accounts_public_id;
DROP INDEX IF EXISTS idx_accounts_trace_id;
DROP INDEX IF EXISTS idx_accounts_ownership_id;

DROP TABLE IF EXISTS accounts;


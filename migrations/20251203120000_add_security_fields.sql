-- Add security and health tracking fields
ALTER TABLE wallets ADD COLUMN device_bound_at DATETIME;
ALTER TABLE wallets ADD COLUMN recovery_phrase_shown INTEGER NOT NULL DEFAULT 0;
ALTER TABLE wallets ADD COLUMN last_seen_at DATETIME;

-- Update existing records
UPDATE wallets SET device_bound_at = created_at WHERE device_bound_at IS NULL;

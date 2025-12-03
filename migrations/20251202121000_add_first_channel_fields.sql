-- 003 - Add first channel tracking fields
ALTER TABLE wallets ADD COLUMN first_channel_opened INTEGER NOT NULL DEFAULT 0; -- 0/1
ALTER TABLE wallets ADD COLUMN first_channel_sats INTEGER NOT NULL DEFAULT 0;   -- i64 satoshis

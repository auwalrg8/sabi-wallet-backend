-- 003 - Add first channel tracking fields
ALTER TABLE wallets ADD first_channel_opened INTEGER DEFAULT 0; -- 0/1
ALTER TABLE wallets ADD first_channel_sats INTEGER DEFAULT 0;   -- i64 satoshis

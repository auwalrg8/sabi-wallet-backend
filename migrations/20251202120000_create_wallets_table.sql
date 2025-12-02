-- 002 - Wallets table + one-device binding
CREATE TABLE IF NOT EXISTS wallets (
    wallet_id TEXT PRIMARY KEY NOT NULL,        -- UUID v7 (our internal ID)
    phone TEXT NOT NULL,                        -- +234xxxxxxxxxx
    device_id TEXT NOT NULL UNIQUE,             -- One device = one wallet forever
    node_pubkey TEXT NOT NULL,                  -- Breez Spark node pubkey
    invite_code TEXT NOT NULL,                  -- SABI-XXXXX code for instant inbound
    status TEXT NOT NULL DEFAULT 'active',      -- active | suspended | closed
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    updated_at DATETIME NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_wallets_phone ON wallets(phone);
CREATE INDEX IF NOT EXISTS idx_wallets_device_id ON wallets(device_id);
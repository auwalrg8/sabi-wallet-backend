-- 002 - Wallets table + one-device binding
CREATE TABLE IF NOT EXISTS wallets (
    wallet_id TEXT PRIMARY KEY NOT NULL,        -- UUID v7 (our internal ID)
    phone TEXT NOT NULL,                        -- +234xxxxxxxxxx
    device_id TEXT NOT NULL UNIQUE,             -- One device = one wallet forever
    breez_node_id TEXT NOT NULL,                -- Breez Spark node id (pubkey)
    invite_code TEXT NOT NULL,                  -- Invite/mnemonic or code from Breez
    backup_type TEXT NOT NULL DEFAULT 'none',   -- none | social | seed
    backup_status TEXT NOT NULL DEFAULT 'skipped', -- skipped | pending | completed
    status TEXT NOT NULL DEFAULT 'active',      -- active | suspended | closed
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    updated_at DATETIME NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_wallets_phone ON wallets(phone);
CREATE INDEX IF NOT EXISTS idx_wallets_device_id ON wallets(device_id);
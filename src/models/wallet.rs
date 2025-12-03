use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Wallet {
    pub wallet_id: String,
    pub phone: String,
    pub device_id: String,
    pub breez_node_id: String,
    pub invite_code: String,
    pub backup_type: String,
    pub backup_status: String,
    pub status: String,
    pub first_channel_opened: i64, // 0 or 1
    pub first_channel_sats: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub device_id: String,
    pub phone: String,  // must be +234...
    #[serde(default = "default_backup_type")]
    pub backup_type: String, // "none" | "social" | "seed"
}

fn default_backup_type() -> String {
    "none".to_string()
}

#[derive(Serialize)]
pub struct CreateWalletResponse {
    pub invite_code: String,
    pub node_id: String,
    pub initial_channel_opened: bool,
}
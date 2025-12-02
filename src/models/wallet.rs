use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Wallet {
    pub wallet_id: String,
    pub phone: String,
    pub device_id: String,
    pub node_pubkey: String,
    pub invite_code: String,
    pub backup_type: String,
    pub backup_status: String,
    pub status: String,
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
    pub wallet_id: String,
    pub invite_code: String,
    pub node_id: String,
}
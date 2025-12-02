use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Wallet {
    pub wallet_id: String,
    pub phone: String,
    pub device_id: String,
    pub node_pubkey: String,
    pub invite_code: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct CreateWalletRequest {
    pub device_id: String,
    pub phone: String,  // must be +234...
}

#[derive(Serialize)]
pub struct CreateWalletResponse {
    pub wallet_id: String,
    pub invite_code: String,
    pub node_id: String,
}
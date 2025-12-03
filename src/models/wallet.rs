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
    pub device_bound_at: Option<chrono::DateTime<chrono::Utc>>,
    pub recovery_phrase_shown: i64, // 0 or 1
    pub last_seen_at: Option<chrono::DateTime<chrono::Utc>>,
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

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: String,
    pub database: ServiceHealth,
    pub breez_service: ServiceHealth,
    pub lsp: ServiceHealth,
}

#[derive(Serialize)]
pub struct ServiceHealth {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

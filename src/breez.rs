use serde::{Deserialize, Serialize};

// Breez SDK Nodeless (Spark) - December 2025 Reality:
// 
// All Lightning wallet operations happen CLIENT-SIDE on the device.
// The backend does NOT create nodes, open channels, or manage Lightning state.
// 
// This module is kept minimal for configuration only.
// Real implementation is in Flutter app using @breeztech/breez-sdk-spark

#[derive(Clone)]
pub struct BreezServices {
    pub api_key: String,
    pub config: NodelessConfig,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodelessConfig {
    pub first_channel_sats_default: Option<i64>,
}

impl BreezServices {
    pub async fn init_nodeless(api_key: String, config: NodelessConfig) -> anyhow::Result<Self> {
        tracing::info!("Breez SDK Nodeless initialized (operations run on device)");
        Ok(Self { api_key, config })
    }
}


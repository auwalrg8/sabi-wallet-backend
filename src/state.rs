use sqlx::SqlitePool;
use crate::config::Config;
use crate::breez::{BreezServices, NodelessConfig};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub breez: Arc<BreezServices>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(&config.database_url).await?;

        // Breez SDK Nodeless (2025) - minimal config
        // API key is used by client SDK, not backend
        let breez = BreezServices::init_nodeless(
            config.breez_api_key.clone(),
            NodelessConfig {
                first_channel_sats_default: config.first_channel_sats_default,
            },
        ).await?;

        Ok(Self {
            db,
            config,
            breez: Arc::new(breez),
        })
    }
}

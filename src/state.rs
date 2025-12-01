use sqlx::SqlitePool;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    // TODO: Add breez SDK integration when available
    // pub breez: Arc<breez_sdk::BreezServices>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(&config.database_url).await?;

        // TODO: Initialize Breez SDK when available
        /*
        let breez = breez_sdk::BreezServices::init(
            breez_sdk::Config {
                api_key: config.breez_api_key.clone(),
                network: breez_sdk::BitcoinNetwork::Bitcoin, // Spark uses Bitcoin mainnet under the hood
                mode: breez_sdk::Mode::Nodeless,
                spark_url: None, // Uses default production Spark
            },
        ).await?;
        */

        Ok(Self {
            db,
            config,
            // breez: Arc::new(breez),
        })
    }
}

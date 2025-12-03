use sqlx::SqlitePool;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let db = SqlitePool::connect(&config.database_url).await?;

        tracing::info!("Backend initialized - metadata-only storage (Lightning on device)");

        Ok(Self {
            db,
            config,
        })
    }
}

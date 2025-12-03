use std::env;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: Option<String>,
    pub first_channel_sats_default: Option<i64>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("database_url", "sqlite:./data/sabi.db?mode=rwc")?
            .set_default("first_channel_sats_default", 200000)?
            .build()?
            .try_deserialize::<Self>()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "file:./sabi.db".into()),
            redis_url: env::var("REDIS_URL").ok(),
            first_channel_sats_default: env::var("FIRST_CHANNEL_SATS_DEFAULT").ok().and_then(|v| v.parse::<i64>().ok()),
        }
    }
}

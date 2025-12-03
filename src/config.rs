use std::env;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub breez_api_key: String,
    pub breez_env: BreezEnv,
    pub database_url: String,
    pub redis_url: Option<String>,
    pub first_channel_sats_default: Option<i64>,
    pub breez_service_url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BreezEnv {
    Production,
    Staging,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .set_default("breez_env", "production")?
            .set_default("database_url", "sqlite:./data/sabi.db?mode=rwc")?
            .set_default("first_channel_sats_default", 200000)?
            .set_default("breez_service_url", "http://localhost:3001")?
            .build()?
            .try_deserialize::<Self>()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            breez_api_key: env::var("BREEZ_API_KEY").expect("BREEZ_API_KEY missing"),
            breez_env: match env::var("BREEZ_ENV").as_deref() {
                Ok("staging") => BreezEnv::Staging,
                _ => BreezEnv::Production,
            },
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "file:./sabi.db".into()),
            redis_url: env::var("REDIS_URL").ok(),
            first_channel_sats_default: env::var("FIRST_CHANNEL_SATS_DEFAULT").ok().and_then(|v| v.parse::<i64>().ok()),
            breez_service_url: env::var("BREEZ_SERVICE_URL").ok(),
        }
    }
}

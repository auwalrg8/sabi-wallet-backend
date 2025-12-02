use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct BreezServices {
	pub api_key: String,
	pub config: NodelessConfig,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodelessConfig {
	pub spark_url: Option<String>,
	pub invite_code_prefix: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
	pub node_pubkey: String,
	pub invite_code: String,
}

impl BreezServices {
	pub async fn init_nodeless(api_key: String, config: NodelessConfig) -> Result<Self> {
		Ok(Self { api_key, config })
	}

	pub async fn create_node(&self) -> Result<NodeInfo> {
		// Placeholder implementation until real SDK is wired
		let node_pubkey = format!("npub1{}", uuid::Uuid::now_v7().simple());
		let invite_suffix = &uuid::Uuid::now_v7().to_string()[..8];
		let prefix = self.config.invite_code_prefix.clone().unwrap_or_else(|| "SABI".to_string());
		let invite_code = format!("{}-{}", prefix, invite_suffix.to_uppercase());
		Ok(NodeInfo { node_pubkey, invite_code })
	}
}

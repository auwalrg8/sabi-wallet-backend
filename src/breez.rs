use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct BreezServices {
    pub api_key: String,
    pub config: NodelessConfig,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NodelessConfig {
    pub spark_url: Option<String>,
    pub invite_code_prefix: Option<String>,
    pub first_channel_sats_default: Option<i64>,
    pub service_url: Option<String>, // Breez microservice URL
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
        // Call Breez microservice to create node
        let service_url = self
            .config
            .service_url
            .clone()
            .unwrap_or_else(|| "http://localhost:3001".to_string());

        #[derive(Serialize)]
        struct CreateNodeReq {
            wallet_id: String,
        }

        #[derive(Deserialize)]
        struct CreateNodeResp {
            node_id: String,
            invite_code: String,
        }

        let wallet_id = uuid::Uuid::now_v7().to_string();
        let url = format!("{}/api/create-node", service_url);
        
        tracing::info!(url, wallet_id, "Calling Breez microservice to create node");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&CreateNodeReq { wallet_id })
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Breez service request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Breez service error: {} - {}", status, body);
        }

        let parsed: CreateNodeResp = resp
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Invalid Breez service response: {e}"))?;

        tracing::info!(node_id = %parsed.node_id, "✅ Breez node created");

        Ok(NodeInfo {
            node_pubkey: parsed.node_id,
            invite_code: parsed.invite_code,
        })
    }

    pub async fn open_first_channel(&self, wallet_id: &str, amount_sats: i64) -> Result<()> {
        // Validate amount within 100k–300k sats
        if amount_sats < 100_000 || amount_sats > 300_000 {
            anyhow::bail!("channel amount must be between 100k and 300k sats");
        }

        // Call Breez microservice to open channel
        let service_url = self
            .config
            .service_url
            .clone()
            .unwrap_or_else(|| "http://localhost:3001".to_string());

        #[derive(Serialize)]
        struct OpenChannelReq {
            wallet_id: String,
            amount_sats: i64,
        }

        let url = format!("{}/api/open-channel", service_url);
        
        tracing::info!(url, wallet_id, amount_sats, "Calling Breez microservice to open channel");

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&OpenChannelReq {
                wallet_id: wallet_id.to_string(),
                amount_sats,
            })
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Breez service request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Breez service error: {} - {}", status, body);
        }

        tracing::info!("✅ Channel opening initiated");
        Ok(())
    }

    pub async fn get_wallet_status(&self, node_id: &str) -> Result<WalletStatus> {
        let service_url = self
            .config
            .service_url
            .clone()
            .unwrap_or_else(|| "http://localhost:3001".to_string());

        let url = format!("{}/api/wallet-status/{}", service_url, node_id);
        
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Breez service request failed: {e}"))?;

        if !resp.status().is_success() {
            return Ok(WalletStatus::default());
        }

        let status: WalletStatus = resp
            .json()
            .await
            .unwrap_or_default();

        Ok(status)
    }

    pub async fn health_check(&self) -> Result<()> {
        let service_url = self
            .config
            .service_url
            .clone()
            .unwrap_or_else(|| "http://localhost:3001".to_string());

        let url = format!("{}/health", service_url);
        
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Breez service health check failed: {e}"))?;

        if !resp.status().is_success() {
            anyhow::bail!("Breez service unhealthy: {}", resp.status());
        }

        Ok(())
    }

    pub async fn check_lsp_status(&self) -> Result<LspStatus> {
        let service_url = self
            .config
            .service_url
            .clone()
            .unwrap_or_else(|| "http://localhost:3001".to_string());

        let url = format!("{}/api/lsp-status", service_url);
        
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("LSP status check failed: {e}"))?;

        if !resp.status().is_success() {
            anyhow::bail!("LSP status check failed: {}", resp.status());
        }

        let status: LspStatus = resp
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Invalid LSP status response: {e}"))?;

        Ok(status)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WalletStatus {
    pub balance_sats: i64,
    pub channel_count: i64,
    pub channel_capacity_sats: i64,
    pub is_connected: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LspStatus {
    pub lsp_id: String,
    pub is_online: bool,
}


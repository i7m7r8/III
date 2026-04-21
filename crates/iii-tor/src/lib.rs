pub mod client;

use arti_client::{TorClient, TorClientConfig};
use tor_rtcompat::tokio::TokioNativeTlsRuntime;
use std::sync::Arc;
use iii_core::error::IIIError;

pub struct TorManager {
    client: Option<Arc<TorClient<TokioNativeTlsRuntime>>>,
}

impl TorManager {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub async fn start(&mut self, data_dir: std::path::PathBuf) -> Result<(), IIIError> {
        let config = TorClientConfig::builder()
            .storage(data_dir)
            .build()
            .map_err(|e| IIIError::Tor(e.to_string()))?;
        let client = TorClient::create_bootstrapped(config).await
            .map_err(|e| IIIError::Tor(e.to_string()))?;
        self.client = Some(Arc::new(client));
        tracing::info!("Tor started and bootstrapped");
        Ok(())
    }

    pub async fn socks_proxy(&self) -> Result<(String, u16), IIIError> {
        // In real implementation, we'd get the SOCKS5 port from arti
        // For now, return a placeholder.
        Ok(("127.0.0.1".to_string(), 19050))
    }
}

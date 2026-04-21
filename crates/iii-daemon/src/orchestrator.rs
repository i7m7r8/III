use anyhow::{Context, Result};
use iii_core::{AppState, Mode};
use iii_i2p::{I2pConfig, I2pInstance};
use iii_sni::SniProxy;
use iii_tor::{TorConfig, TorInstance};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

pub struct ChainManager {
    state: Arc<AppState>,
    tor: Arc<Mutex<Option<TorInstance>>>,
    i2p: Arc<Mutex<Option<I2pInstance>>>,
}

impl ChainManager {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            tor: Arc::new(Mutex::new(None)),
            i2p: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn stop_chain(&self) -> Result<()> {
        let mut tor_guard = self.tor.lock().await;
        if let Some(tor) = tor_guard.take() {
            tor.stop().await?;
        }
        let mut i2p_guard = self.i2p.lock().await;
        if let Some(i2p) = i2p_guard.take() {
            i2p.stop().await?;
        }
        Ok(())
    }

    pub async fn start_chain(&self) -> Result<()> {
        let mode = *self.state.mode.read().await;
        let sni_domain = self.state.sni_domain.read().await.clone();
        let target_relay = self.state.target_relay.read().await.clone();

        info!("Starting chain with mode: {:?}", mode);

        // 1. Always start SNI Proxy
        let sni_proxy = SniProxy::new("127.0.0.1:10000".to_string(), sni_domain, target_relay)?;

        tokio::spawn(async move {
            if let Err(e) = sni_proxy.run().await {
                error!("SNI Proxy failed: {}", e);
            }
        });

        match mode {
            Mode::SniOnly => {
                info!("Chain: APP -> SNI -> INTERNET");
            }
            Mode::SniTor => {
                info!("Chain: APP -> TOR -> SNI -> INTERNET");
                let tor = TorInstance::new(None)?;
                tor.start(TorConfig {
                    socks_port: 9050,
                    control_port: 9051,
                    data_dir: tor.data_dir(),
                    upstream_proxy: Some(("127.0.0.1".to_string(), 10000)),
                })
                .await?;
                *self.tor.lock().await = Some(tor);
            }
            Mode::SniI2p => {
                info!("Chain: APP -> I2P -> SNI -> INTERNET");
                let i2p = I2pInstance::new(None)?;
                i2p.start(I2pConfig {
                    http_proxy_port: 4444,
                    socks_proxy_port: 4447,
                    data_dir: i2p.data_dir(),
                    upstream_proxy: Some(("127.0.0.1".to_string(), 10000)),
                })
                .await?;
                *self.i2p.lock().await = Some(i2p);
            }
            Mode::SniTorI2p => {
                info!("Chain: APP -> TOR -> I2P -> SNI -> INTERNET");
                let i2p = I2pInstance::new(None)?;
                i2p.start(I2pConfig {
                    http_proxy_port: 4444,
                    socks_proxy_port: 4447,
                    data_dir: i2p.data_dir(),
                    upstream_proxy: Some(("127.0.0.1".to_string(), 10000)),
                })
                .await?;
                *self.i2p.lock().await = Some(i2p);

                let tor = TorInstance::new(None)?;
                tor.start(TorConfig {
                    socks_port: 9050,
                    control_port: 9051,
                    data_dir: tor.data_dir(),
                    upstream_proxy: Some(("127.0.0.1".to_string(), 4447)),
                })
                .await?;
                *self.tor.lock().await = Some(tor);
            }
            Mode::SniI2pTor => {
                info!("Chain: APP -> I2P -> TOR -> SNI -> INTERNET");
                let tor = TorInstance::new(None)?;
                tor.start(TorConfig {
                    socks_port: 9050,
                    control_port: 9051,
                    data_dir: tor.data_dir(),
                    upstream_proxy: Some(("127.0.0.1".to_string(), 10000)),
                })
                .await?;
                *self.tor.lock().await = Some(tor);

                let i2p = I2pInstance::new(None)?;
                i2p.start(I2pConfig {
                    http_proxy_port: 4444,
                    socks_proxy_port: 4447,
                    data_dir: i2p.data_dir(),
                    upstream_proxy: Some(("127.0.0.1".to_string(), 9050)),
                })
                .await?;
                *self.i2p.lock().await = Some(i2p);
            }
        }

        Ok(())
    }
}

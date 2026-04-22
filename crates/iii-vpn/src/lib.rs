pub mod killswitch;
pub mod router;
pub mod tun_management;

use anyhow::{Context, Result};
use iii_core::AppState;
use killswitch::KillSwitch;
use std::sync::Arc;
use tracing::{error, info};
use tun_management::{TunDevice, TunInterface};

pub struct VpnController {
    state: Arc<AppState>,
    killswitch: Arc<tokio::sync::Mutex<Option<KillSwitch>>>,
}

impl VpnController {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            killswitch: Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting VPN Controller...");

        let target_relay = self.state.target_relay.read().await.clone();
        let relay_ip = target_relay.split(':').next().unwrap_or("").to_string();

        let ks = KillSwitch::new(relay_ip);
        ks.enable().await.context("Failed to enable killswitch")?;
        *self.killswitch.lock().await = Some(ks);

        let dev = TunInterface::create("iii0", "10.0.0.1", "255.255.255.0")
            .await
            .context("Failed to create TUN interface")?;

        let router = router::Router::new(dev);
        tokio::spawn(async move {
            if let Err(e) = router.run().await {
                error!("VPN Router failed: {}", e);
            }
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        info!("Stopping VPN Controller...");
        let mut ks_guard = self.killswitch.lock().await;
        if let Some(ks) = ks_guard.take() {
            ks.disable().await?;
        }
        Ok(())
    }
}

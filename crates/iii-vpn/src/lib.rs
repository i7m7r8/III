pub mod killswitch;
pub mod router;
pub mod tun;

use anyhow::{Context, Result};
use iii_core::AppState;
use killswitch::KillSwitch;
use std::sync::Arc;
use tracing::{error, info};

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

        // 1. Initialize Killswitch
        let target_relay = self.state.target_relay.read().await.clone();
        let relay_ip = target_relay.split(':').next().unwrap_or("").to_string();

        let ks = KillSwitch::new(relay_ip);
        ks.enable().await.context("Failed to enable killswitch")?;
        *self.killswitch.lock().await = Some(ks);

        // 2. Create TUN Interface
        #[cfg(target_os = "linux")]
        {
            let mut config = tun::Configuration::default();
            config
                .name("iii0")
                .address("10.0.0.1")
                .netmask("255.255.255.0")
                .up();

            let _dev = tun::create(&config).context("Failed to create TUN device")?;
            info!("TUN device 'iii0' created (10.0.0.1)");

            // Production: Start packet forwarder task here
        }

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

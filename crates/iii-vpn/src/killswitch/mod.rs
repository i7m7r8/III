use anyhow::Result;
use tracing::{info, warn};

#[cfg(target_os = "linux")]
use tokio::process::Command;

#[cfg(target_os = "windows")]
mod windows_wfp;

pub struct KillSwitch {
    target_relay_ip: String,
    #[cfg(target_os = "windows")]
    engine_handle: std::sync::Arc<tokio::sync::Mutex<Option<isize>>>,
}

impl KillSwitch {
    pub fn new(target_relay_ip: String) -> Self {
        Self { 
            target_relay_ip,
            #[cfg(target_os = "windows")]
            engine_handle: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        }
    }

    pub async fn enable(&self) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            info!("Enabling nftables killswitch (Fail-Closed)");
            // ... (rest of linux nftables logic)
            self.enable_linux().await
        }
        #[cfg(target_os = "windows")]
        {
            info!("Enabling Windows WFP killswitch (Fail-Closed)");
            let mut handle_guard = self.engine_handle.lock().await;
            let handle = windows_wfp::enable_wfp_killswitch(&self.target_relay_ip).await?;
            *handle_guard = Some(handle);
            Ok(())
        }
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            warn!("Killswitch not implemented for this OS");
            Ok(())
        }
    }

    #[cfg(target_os = "linux")]
    async fn enable_linux(&self) -> Result<()> {
        // 1. Create the table
        let commands = [
            "nft add table inet iii_killswitch",
            "nft flush table inet iii_killswitch",
            // 2. Create the chain
            "nft add chain inet iii_killswitch base_chain { type filter hook output priority 0; policy drop; }",
            // 3. Allow loopback
            "nft add rule inet iii_killswitch base_chain oifname \"lo\" accept",
            // 4. Allow traffic to the SNI Relay (The only exit)
            &format!("nft add rule inet iii_killswitch base_chain ip daddr {} accept", self.target_relay_ip),
            // 5. Allow traffic to the TUN interface
            "nft add rule inet iii_killswitch base_chain oifname \"iii0\" accept",
        ];

        for cmd in commands {
            let status = Command::new("sh").arg("-c").arg(cmd).status().await?;
            if !status.success() {
                warn!("Command failed: {}", cmd);
            }
        }
        Ok(())
    }

    pub async fn disable(&self) -> Result<()> {
        info!("Disabling killswitch");
        #[cfg(target_os = "linux")]
        {
            Command::new("nft")
                .arg("delete")
                .arg("table")
                .arg("inet")
                .arg("iii_killswitch")
                .status()
                .await?;
        }
        #[cfg(target_os = "windows")]
        {
            let mut handle_guard = self.engine_handle.lock().await;
            if let Some(handle) = handle_guard.take() {
                windows_wfp::disable_wfp_killswitch(handle).await?;
            }
        }
        Ok(())
    }
}

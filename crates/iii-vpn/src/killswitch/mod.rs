use anyhow::Result;
use tokio::process::Command;
use tracing::{info, warn};

pub struct KillSwitch {
    target_relay_ip: String,
}

impl KillSwitch {
    pub fn new(target_relay_ip: String) -> Self {
        Self { target_relay_ip }
    }

    pub async fn enable(&self) -> Result<()> {
        info!("Enabling nftables killswitch (Fail-Closed)");
        
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
        Command::new("nft").arg("delete").arg("table").arg("inet").arg("iii_killswitch").status().await?;
        Ok(())
    }
}

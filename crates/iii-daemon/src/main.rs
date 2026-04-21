mod orchestrator;

use iii_core::AppState;
use iii_vpn::VpnController;
use std::sync::Arc;
use orchestrator::ChainManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("Starting III Military Grade VPN Daemon");

    let state = Arc::new(AppState::default());
    
    // 1. Initialize Chain Manager (SNI + Tor + I2P)
    let chain_manager = ChainManager::new(state.clone());
    chain_manager.start_chain().await?;

    // 2. Initialize VPN Controller (TUN/Routing)
    let vpn = VpnController::new(state.clone());
    vpn.start().await?;

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    
    info!("Shutting down...");
    vpn.stop().await?;
    
    Ok(())
}

use tracing::info;

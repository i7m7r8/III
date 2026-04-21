use iii_core::AppState;
use iii_vpn::VpnController;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let state = Arc::new(AppState::default());
    let vpn = VpnController::new(state.clone());
    // Start gRPC server, wait for commands
    tokio::signal::ctrl_c().await?;
    vpn.stop().await?;
    Ok(())
}

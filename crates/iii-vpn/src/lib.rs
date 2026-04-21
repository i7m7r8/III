pub mod tun;
pub mod killswitch;
pub mod router;

use std::sync::Arc;
use iii_core::AppState;

pub struct VpnController {
    state: Arc<AppState>,
}

impl VpnController {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    pub async fn start(&self) -> Result<(), iii_core::error::IIIError> {
        // Implementation will create TUN interface, set up routing,
        // and start the packet forwarder through the selected chain.
        Ok(())
    }

    pub async fn stop(&self) -> Result<(), iii_core::error::IIIError> {
        Ok(())
    }
}

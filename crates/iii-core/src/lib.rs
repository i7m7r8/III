pub mod error;
pub mod config;
pub mod tunnel;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    SniOnly,
    SniTor,
    SniI2p,
    SniTorI2p,
    SniI2pTor,
}

pub struct AppState {
    pub mode: Arc<RwLock<Mode>>,
    pub sni_domain: Arc<RwLock<String>>,
    pub target_relay: Arc<RwLock<String>>,
    pub running: Arc<RwLock<bool>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: Arc::new(RwLock::new(Mode::SniTor)),
            sni_domain: Arc::new(RwLock::new("fronting.cdn.com".to_string())),
            target_relay: Arc::new(RwLock::new("1.2.3.4:443".to_string())),
            running: Arc::new(RwLock::new(false)),
        }
    }
}

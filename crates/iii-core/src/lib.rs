pub mod error;
pub mod config;
pub mod tunnel;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    TorOnly,
    I2POnly,
    BothI2PThenTor,
}

pub struct AppState {
    pub mode: Arc<RwLock<Mode>>,
    pub sni_domain: Arc<RwLock<String>>,
    pub running: Arc<RwLock<bool>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: Arc::new(RwLock::new(Mode::TorOnly)),
            sni_domain: Arc::new(RwLock::new("fronting.cdn.com".to_string())),
            running: Arc::new(RwLock::new(false)),
        }
    }
}

pub mod ffi;
pub mod manager;

use iii_core::error::IIIError;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TorManager {
    inner: Arc<Mutex<manager::TorInner>>,
}

impl TorManager {
    pub fn new(data_dir: PathBuf, socks_port: u16, control_port: u16) -> Self {
        Self {
            inner: Arc::new(Mutex::new(manager::TorInner::new(
                data_dir,
                socks_port,
                control_port,
            ))),
        }
    }

    pub async fn start(&self, upstream_proxy: Option<(String, u16)>) -> Result<(), IIIError> {
        let mut inner = self.inner.lock().await;
        inner.start(upstream_proxy).await
    }

    pub async fn stop(&self) -> Result<(), IIIError> {
        let mut inner = self.inner.lock().await;
        inner.stop().await
    }
}

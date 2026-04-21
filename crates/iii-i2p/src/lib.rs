use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::{Command, Child};
use std::process::Stdio;
use anyhow::{Result, Context};
use tempfile::TempDir;
use std::fs;
use tokio::sync::Mutex;

pub struct I2pConfig {
    pub http_proxy_port: u16,
    pub socks_proxy_port: u16,
    pub data_dir: PathBuf,
    pub upstream_proxy: Option<(String, u16)>, // For SNI/Tor chaining
}

pub struct I2pInstance {
    _temp_dir: Option<TempDir>,
    data_dir: PathBuf,
    child: Arc<Mutex<Option<Child>>>,
}

impl I2pInstance {
    pub fn new(data_dir: Option<PathBuf>) -> Result<Self> {
        let (temp_dir, path) = if let Some(d) = data_dir {
            (None, d)
        } else {
            let td = TempDir::new()?;
            let p = td.path().to_path_buf();
            (Some(td), p)
        };

        Ok(Self {
            _temp_dir: temp_dir,
            data_dir: path,
            child: Arc::new(Mutex::new(None)),
        })
    }

    pub fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    pub async fn stop(&self) -> Result<()> {
        let mut child_guard = self.child.lock().await;
        if let Some(mut child) = child_guard.take() {
            child.kill().await?;
            tracing::info!("i2pd process killed");
        }
        Ok(())
    }

    pub async fn start(&self, config: I2pConfig) -> Result<()> {
        let mut child_guard = self.child.lock().await;
        if child_guard.is_some() {
            return Ok(());
        }

        let conf_path = self.data_dir.join("i2pd.conf");
        let mut conf_content = format!(
            "httpproxy.port = {}\nsocksproxy.port = {}\ndatadir = {}\n",
            config.http_proxy_port,
            config.socks_proxy_port,
            self.data_dir.display()
        );

        if let Some((host, port)) = config.upstream_proxy {
            conf_content.push_str(&format!("httpproxy.proxy = http://{}:{}\n", host, port));
        }

        fs::write(&conf_path, conf_content).context("Failed to write i2pd.conf")?;

        let child = Command::new("i2pd")
            .arg("--conf")
            .arg(&conf_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn i2pd process")?;

        *child_guard = Some(child);
        Ok(())
    }
}

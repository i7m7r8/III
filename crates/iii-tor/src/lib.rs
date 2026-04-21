use std::path::PathBuf;
use std::sync::Arc;
use tokio::process::{Command, Child};
use std::process::Stdio;
use anyhow::{Result, Context};
use tempfile::TempDir;
use std::fs;
use tokio::sync::Mutex;

pub struct TorConfig {
    pub socks_port: u16,
    pub control_port: u16,
    pub data_dir: PathBuf,
    pub upstream_proxy: Option<(String, u16)>,
}

pub struct TorInstance {
    _temp_dir: Option<TempDir>,
    data_dir: PathBuf,
    child: Arc<Mutex<Option<Child>>>,
}

impl TorInstance {
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
            tracing::info!("Tor process killed");
        }
        Ok(())
    }

    pub async fn start(&self, config: TorConfig) -> Result<()> {
        let mut child_guard = self.child.lock().await;
        if child_guard.is_some() {
            return Ok(());
        }

        let torrc_path = self.data_dir.join("torrc");
        let mut torrc_content = format!(
            "SocksPort {}\nControlPort {}\nDataDirectory {}\n",
            config.socks_port,
            config.control_port,
            self.data_dir.display()
        );

        if let Some((host, port)) = config.upstream_proxy {
            torrc_content.push_str(&format!("HTTPSProxy {}:{}\n", host, port));
        }

        fs::write(&torrc_path, torrc_content).context("Failed to write torrc")?;

        let child = Command::new("tor")
            .arg("-f")
            .arg(&torrc_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn tor process")?;

        *child_guard = Some(child);
        Ok(())
    }
}

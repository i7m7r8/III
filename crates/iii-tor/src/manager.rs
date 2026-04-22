use std::path::PathBuf;
use std::process::{Command, Child};
use tracing::{info};
use iii_core::error::IIIError;

pub struct TorInner {
    data_dir: PathBuf,
    socks_port: u16,
    control_port: u16,
    child: Option<Child>,
}

impl TorInner {
    pub fn new(data_dir: PathBuf, socks_port: u16, control_port: u16) -> Self {
        Self {
            data_dir,
            socks_port,
            control_port,
            child: None,
        }
    }

    pub async fn start(&mut self, upstream_proxy: Option<(String, u16)>) -> Result<(), IIIError> {
        if self.child.is_some() {
            return Err(IIIError::AlreadyRunning);
        }

        let torrc_path = self.data_dir.join("torrc");
        let mut torrc_content = format!(
            "SocksPort {}\nControlPort {}\nDataDirectory {}\n",
            self.socks_port, self.control_port, self.data_dir.display()
        );

        if let Some((host, port)) = upstream_proxy {
            torrc_content.push_str(&format!("Socks5Proxy {}:{}\n", host, port));
        }

        std::fs::write(&torrc_path, torrc_content).map_err(|e| IIIError::Tor(e.to_string()))?;

        #[cfg(target_os = "android")]
        let tor_bin = {
            let possible_paths = [
                "/data/local/tmp/tor",
                "./tor",
                "tor",
            ];
            let mut resolved = "tor".to_string();
            for path in &possible_paths {
                if std::fs::metadata(path).is_ok() {
                    resolved = path.to_string();
                    break;
                }
            }
            resolved
        };
        #[cfg(not(target_os = "android"))]
        let tor_bin = "tor";

        let child = Command::new(&tor_bin)
            .arg("-f")
            .arg(&torrc_path)
            .spawn()
            .map_err(|e| IIIError::Tor(e.to_string()))?;

        self.child = Some(child);
        info!("Tor started on socks port {}", self.socks_port);
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), IIIError> {
        if let Some(mut child) = self.child.take() {
            child.kill().map_err(|e| IIIError::Tor(e.to_string()))?;
            child.wait().map_err(|e| IIIError::Tor(e.to_string()))?;
            info!("Tor stopped");
        }
        Ok(())
    }
}

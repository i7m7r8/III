use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IIIConfig {
    pub sni_domain: String,
    pub mode: String, // "tor", "i2p", "both"
    pub order: String, // "i2p_then_tor" or "tor_then_i2p"
    pub killswitch_enabled: bool,
    pub dns_through_tunnel: bool,
    pub data_dir: PathBuf,
}

impl Default for IIIConfig {
    fn default() -> Self {
        Self {
            sni_domain: "fronting.cdn.com".to_string(),
            mode: "tor".to_string(),
            order: "i2p_then_tor".to_string(),
            killswitch_enabled: true,
            dns_through_tunnel: true,
            data_dir: dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".")).join("iii"),
        }
    }
}

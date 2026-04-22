use thiserror::Error;

#[derive(Error, Debug)]
pub enum IIIError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),

    #[error("Tor error: {0}")]
    Tor(String),

    #[error("I2P error: {0}")]
    I2p(String),

    #[error("VPN error: {0}")]
    Vpn(String),

    #[error("Invalid SNI domain")]
    InvalidSni,

    #[error("Kill switch failed: {0}")]
    KillSwitchFailed(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Process already running")]
    AlreadyRunning,
}

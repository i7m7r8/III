pub mod dialer;

use iii_core::tunnel::{Tunnel, SniTunnel as CoreSniTunnel};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use rustls::{ClientConfig, ServerName};
use std::sync::Arc as StdArc;

#[derive(Clone)]
pub struct SniTunnel {
    sni_domain: Arc<String>,
    config: StdArc<ClientConfig>,
}

impl SniTunnel {
    pub fn new(sni_domain: String) -> Result<Self, iii_core::error::IIIError> {
        let mut config = ClientConfig::builder()
            .with_root_certificates(rustls::RootCertStore {
                roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
            })
            .with_no_client_auth();
        config.enable_sni = true;
        Ok(Self {
            sni_domain: Arc::new(sni_domain),
            config: StdArc::new(config),
        })
    }

    pub async fn connect(&self, target: &str) -> Result<tokio_rustls::client::TlsStream<TcpStream>, iii_core::error::IIIError> {
        let server_name = ServerName::try_from(self.sni_domain.as_str())
            .map_err(|_| iii_core::error::IIIError::InvalidSni)?;
        let tcp = TcpStream::connect(target).await?;
        let connector = TlsConnector::from(self.config.clone());
        let stream = connector.connect(server_name, tcp).await?;
        Ok(stream)
    }
}

use anyhow::{Context, Result};
use rustls::ClientConfig;
use rustls_pki_types::ServerName;
use std::sync::Arc;
use tokio::io::{copy_bidirectional, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::TlsConnector;
use tracing::{error, info};

pub struct SniProxy {
    listen_addr: String,
    sni_domain: String,
    target_relay: String,
    tls_config: Arc<ClientConfig>,
}

impl SniProxy {
    pub fn new(listen_addr: String, sni_domain: String, target_relay: String) -> Result<Self> {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let mut config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        config.enable_sni = true;

        Ok(Self {
            listen_addr,
            sni_domain,
            target_relay,
            tls_config: Arc::new(config),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.listen_addr).await?;
        info!("SNI Proxy (SOCKS5) listening on {}", self.listen_addr);

        loop {
            let (client_stream, addr) = listener.accept().await?;
            let sni_domain = self.sni_domain.clone();
            let target_relay = self.target_relay.clone();
            let tls_config = self.tls_config.clone();

            tokio::spawn(async move {
                if let Err(e) =
                    Self::handle_client(client_stream, sni_domain, target_relay, tls_config).await
                {
                    error!("Error handling client {}: {}", addr, e);
                }
            });
        }
    }

    async fn handle_client(
        mut client_stream: TcpStream,
        sni_domain: String,
        target_relay: String,
        tls_config: Arc<ClientConfig>,
    ) -> Result<()> {
        // 1. SOCKS5 Handshake
        let mut buf = [0u8; 2];
        client_stream.read_exact(&mut buf).await?;
        if buf[0] != 0x05 {
            return Err(anyhow::anyhow!("Only SOCKS5 is supported"));
        }
        let nmethods = buf[1] as usize;
        let mut methods = vec![0u8; nmethods];
        client_stream.read_exact(&mut methods).await?;
        // No auth for now
        client_stream.write_all(&[0x05, 0x00]).await?;

        // 2. SOCKS5 Request
        let mut req_header = [0u8; 4];
        client_stream.read_exact(&mut req_header).await?;
        if req_header[1] != 0x01 {
            // Only CONNECT
            return Err(anyhow::anyhow!("Only SOCKS5 CONNECT is supported"));
        }

        // Skip address parsing for now, we wrap everything to the target relay
        // Production grade should parse target to support multiple relays or direct SNI
        let mut addr_buf = match req_header[3] {
            0x01 => vec![0u8; 4], // IPv4
            0x03 => {
                let len = client_stream.read_u8().await? as usize;
                vec![0u8; len]
            }
            0x04 => vec![0u8; 16], // IPv6
            _ => return Err(anyhow::anyhow!("Invalid address type")),
        };
        client_stream.read_exact(&mut addr_buf).await?;
        let mut port_buf = [0u8; 2];
        client_stream.read_exact(&mut port_buf).await?;

        // Reply success
        client_stream
            .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
            .await?;

        // 3. Wrap outgoing connection to relay in TLS with SNI
        let connector = TlsConnector::from(tls_config);
        let relay_tcp = TcpStream::connect(&target_relay)
            .await
            .context("Failed to connect to SNI relay")?;

        let server_name = ServerName::try_from(sni_domain.as_str())
            .map_err(|_| anyhow::anyhow!("Invalid SNI domain"))?
            .to_owned();

        let mut relay_tls = connector
            .connect(server_name, relay_tcp)
            .await
            .context("Failed TLS handshake with SNI relay")?;

        // 4. Bidirectional copy
        copy_bidirectional(&mut client_stream, &mut relay_tls).await?;

        Ok(())
    }
}

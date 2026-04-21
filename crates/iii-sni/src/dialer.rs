// Advanced SNI dialer with retries and timeout
use tokio::time::{timeout, Duration};
use super::SniTunnel;

impl SniTunnel {
    pub async fn dial_with_retry(&self, target: &str, max_retries: u32) -> Result<tokio_rustls::client::TlsStream<tokio::net::TcpStream>, iii_core::error::IIIError> {
        let mut last_err = None;
        for _ in 0..max_retries {
            match timeout(Duration::from_secs(10), self.connect(target)).await {
                Ok(Ok(stream)) => return Ok(stream),
                Ok(Err(e)) => last_err = Some(e),
                Err(_) => last_err = Some(iii_core::error::IIIError::Io(std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout"))),
            }
        }
        Err(last_err.unwrap_or_else(|| iii_core::error::IIIError::Io(std::io::Error::new(std::io::ErrorKind::Other, "all retries failed"))))
    }
}

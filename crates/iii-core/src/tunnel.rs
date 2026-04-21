use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};

/// A generic tunnel that can be chained.
pub trait Tunnel: Send + Sync {
    type Stream: AsyncRead + AsyncWrite + Unpin + Send;
    async fn connect(&self, target: &str) -> Result<Self::Stream, crate::error::IIIError>;
}

/// A placeholder for the actual SNI tunnel implementation.
pub struct SniTunnel {
    pub sni: Arc<String>,
}

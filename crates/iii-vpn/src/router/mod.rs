use crate::tun_management::TunDevice;
use anyhow::Result;
use tracing::info;

pub struct Router {
    tun: TunDevice,
}

impl Router {
    pub fn new(tun: TunDevice) -> Self {
        Self { tun }
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting packet router loop...");

        match self.tun {
            #[cfg(target_os = "linux")]
            TunDevice::Linux(mut dev) => {
                use tokio::io::AsyncReadExt;
                let mut buf = [0u8; 1600];
                loop {
                    let n = dev.read(&mut buf).await?;
                    if n == 0 {
                        break;
                    }
                    let _packet = &buf[..n];
                    // TODO: Process packet with smoltcp and proxy to upstream
                }
            }
            #[cfg(target_os = "windows")]
            TunDevice::Windows(adapter) => {
                let mut session = adapter.start_session(::wintun::MAX_RING_CAPACITY)?;
                loop {
                    let packet = session.receive_blocking()?;
                    let _data = packet.bytes();
                    // TODO: Process packet with smoltcp and proxy to upstream
                }
            }
        }

        Ok(())
    }
}

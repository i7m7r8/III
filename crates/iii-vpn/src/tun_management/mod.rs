use anyhow::{Context, Result};
use tracing::info;

pub struct TunInterface;

pub enum TunDevice {
    #[cfg(target_os = "linux")]
    Linux(::tun::AsyncDevice),
    #[cfg(target_os = "windows")]
    Windows(std::sync::Arc<::wintun::Adapter>),
}

impl TunInterface {
    pub async fn create(name: &str, address: &str, netmask: &str) -> Result<TunDevice> {
        #[cfg(target_os = "linux")]
        {
            let mut config = ::tun::Configuration::default();
            config
                .name(name)
                .address(address)
                .netmask(netmask)
                .up();

            let dev = ::tun::create_as_async(&config).context("Failed to create TUN device on Linux")?;
            info!("TUN device '{}' created ({})", name, address);
            Ok(TunDevice::Linux(dev))
        }

        #[cfg(target_os = "windows")]
        {
            let wintun = unsafe { ::wintun::load()? };
            let adapter = ::wintun::Adapter::create(&wintun, "III", name, None)
                .context("Failed to create Wintun adapter")?;
            
            adapter.set_address(address.parse()?)?;
            adapter.set_netmask(netmask.parse()?)?;
            
            info!("Wintun device '{}' created ({})", name, address);
            Ok(TunDevice::Windows(std::sync::Arc::new(adapter)))
        }
    }
}

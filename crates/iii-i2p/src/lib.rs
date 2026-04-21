#[cxx::bridge]
mod ffi {
    extern "C++" {
        unsafe fn i2pd_start(port: u16) -> bool;
        unsafe fn i2pd_stop();
        unsafe fn i2pd_set_socks_proxy(addr: *const libc::c_char);
    }
}

pub mod manager;

pub struct I2pManager {
    running: bool,
}

impl I2pManager {
    pub fn new() -> Self {
        Self { running: false }
    }

    pub unsafe fn start(&mut self, socks_port: u16) -> Result<(), iii_core::error::IIIError> {
        if ffi::i2pd_start(socks_port) {
            self.running = true;
            Ok(())
        } else {
            Err(iii_core::error::IIIError::I2p("failed to start i2pd".into()))
        }
    }

    pub fn stop(&mut self) {
        unsafe { ffi::i2pd_stop() }
        self.running = false;
    }
}

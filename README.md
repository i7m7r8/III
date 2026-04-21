# III – Military‑Grade VPN with SNI → I2P / Tor Chaining

**Build status**: [GitHub Actions](https://github.com/i7m7r8/III/actions)

## Overview

III is a cross‑platform (Android, Windows, Linux) VPN that first establishes a TLS tunnel to a user‑defined SNI domain, then routes all traffic through I2P and/or Tor. It is built in Rust for memory safety and high assurance.

## Features

- **SNI fronting** – Bypass DPI by hiding the true destination.
- **Embedded Tor (arti)** – No external binary.
- **Embedded I2P (i2pd)** – Static FFI bindings.
- **Kill switch** – Platform firewall blocks leaks.
- **Cross‑platform GUI** – egui.
- **Reproducible builds** – GitHub Actions with signed artifacts.

## Build from source

See [BUILD.md](BUILD.md) (to be written). For end users, download from [Releases](https://github.com/i7m7r8/III/releases).

## License

GPL‑3.0‑or‑later

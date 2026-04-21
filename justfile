# Default target
default:
    just --list

# Run lints
lint:
    cargo fmt --check
    cargo clippy --workspace -- -D warnings

# Run tests
test:
    cargo test --workspace

# Build release (native)
build-release:
    cargo build --release

# Run daemon locally (Linux only)
run-daemon:
    sudo target/release/iii-daemon

# Generate gRPC bindings
gen-proto:
    cargo run --manifest-path crates/iii-control/build.rs

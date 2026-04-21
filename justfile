default:
    just --list

lint:
    cargo fmt --check
    cargo clippy --workspace -- -D warnings

test:
    cargo test --workspace

build-release:
    cargo build --release

run-daemon:
    sudo target/release/iii-daemon

gen-proto:
    cargo run --manifest-path crates/iii-control/build.rs

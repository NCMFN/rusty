#!/bin/bash
cargo fmt --manifest-path hlcs-hft/Cargo.toml
cargo clippy --manifest-path hlcs-hft/Cargo.toml -- -D warnings
cargo test --manifest-path hlcs-hft/Cargo.toml

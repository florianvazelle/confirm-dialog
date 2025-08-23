default:
    just --list

lint:
    cargo fmt --check
    cargo check
    cargo clippy -- --deny warnings

fmt:
    cargo fmt
    cargo fix --allow-dirty
    cargo clippy --fix --allow-dirty -- --deny warnings

build:
    cargo build
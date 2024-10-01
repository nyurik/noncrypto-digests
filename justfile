#!/usr/bin/env just --justfile

@_default:
    just --list --unsorted

# Clean all build artifacts
clean:
    cargo clean
    rm -f Cargo.lock

update:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

build:
    cargo build --all-targets

# Run cargo clippy
clippy:
    cargo clippy --all-targets -- -D warnings
    cargo clippy --all-targets --all-features -- -D warnings

# Test code formatting
test-fmt:
    cargo fmt --all -- --check

# Run cargo fmt
fmt:
    cargo +nightly fmt -- --config imports_granularity=Module,group_imports=StdExternalCrate

# Build and open code documentation
docs:
    cargo doc --no-deps --open

# Quick compile
check:
    RUSTFLAGS='-D warnings' cargo check --workspace --all-targets

# Run all tests
test:
    RUSTFLAGS='-D warnings' cargo test --all-targets --all-features
    RUSTFLAGS='-D warnings' cargo test --all-targets --no-default-features --features fnv
    RUSTFLAGS='-D warnings' cargo test --all-targets --no-default-features --features xxh3
    RUSTFLAGS='-D warnings' cargo test --all-targets --no-default-features --features xxh32
    RUSTFLAGS='-D warnings' cargo test --all-targets --no-default-features --features xxh64

# Test documentation
test-doc:
    RUSTFLAGS='-D warnings' cargo test --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

rust-info:
    rustc --version
    cargo --version

# Run all tests as expected by CI
ci-test: rust-info test-fmt clippy check test test-doc

# Run integration tests and save its output as the new expected output
bless *ARGS: (cargo-install "insta" "cargo-insta")
    cargo insta test --accept --unreferenced=auto --all-features {{ ARGS }}

# Check if a certain Cargo command is installed, and install it if needed
[private]
cargo-install $COMMAND $INSTALL_CMD="" *ARGS="":
    @if ! command -v $COMMAND > /dev/null; then \
        echo "$COMMAND could not be found. Installing it with    cargo install ${INSTALL_CMD:-$COMMAND} {{ ARGS }}" ;\
        cargo install ${INSTALL_CMD:-$COMMAND} {{ ARGS }} ;\
    fi

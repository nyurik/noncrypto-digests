#!/usr/bin/env just --justfile

@_default:
    just --list --unsorted

# Clean all build artifacts
clean:
    cargo clean
    rm -f Cargo.lock

# Run cargo fmt and cargo clippy
lint: test-fmt clippy

# Run cargo clippy
clippy:
    cargo clippy --bins --tests --lib --benches --examples -- -D warnings
    cargo clippy --all-features -- -D warnings

# Test code formatting
test-fmt:
    cargo fmt --all -- --check

# Run cargo fmt
fmt:
    cargo +nightly fmt -- --config imports_granularity=Module,group_imports=StdExternalCrate

# Build and open code documentation
docs:
    cargo doc --no-deps --open

# Run all tests
test:
    RUSTFLAGS='-D warnings' cargo test --lib --bins --examples --tests --benches --all-features
    RUSTFLAGS='-D warnings' cargo test --lib --bins --examples --tests --benches --no-default-features --features fnv
    RUSTFLAGS='-D warnings' cargo test --lib --bins --examples --tests --benches --no-default-features --features xxh3
    RUSTFLAGS='-D warnings' cargo test --lib --bins --examples --tests --benches --no-default-features --features xxh32
    RUSTFLAGS='-D warnings' cargo test --lib --bins --examples --tests --benches --no-default-features --features xxh64

# Test documentation
test-doc:
    cargo test --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Run all tests as expected by CI
ci-test: && lint test test-doc
    rustc --version
    cargo --version

# Run integration tests and save its output as the new expected output
bless *ARGS: (cargo-install "insta" "cargo-insta")
    cargo insta test --accept --unreferenced=auto --all-features {{ ARGS }}

# Check if a certain Cargo command is installed, and install it if needed
[private]
cargo-install $COMMAND $INSTALL_CMD="" *ARGS="":
    @if ! command -v $COMMAND &> /dev/null; then \
        echo "$COMMAND could not be found. Installing it with    cargo install ${INSTALL_CMD:-$COMMAND} {{ ARGS }}" ;\
        cargo install ${INSTALL_CMD:-$COMMAND} {{ ARGS }} ;\
    fi

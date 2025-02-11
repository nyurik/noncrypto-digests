#!/usr/bin/env just --justfile

@_default:
    just --list

# Clean all build artifacts
clean:
    cargo clean
    rm -f Cargo.lock

# Update dependencies, including breaking changes
update:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

# Find the minimum supported Rust version (MSRV) using cargo-msrv extension, and update Cargo.toml
msrv:
    cargo msrv find --write-msrv

build:
    cargo build --workspace --all-targets

# Run cargo clippy
clippy:
    cargo clippy --workspace --all-targets -- -D warnings
    cargo clippy --workspace --all-targets --all-features -- -D warnings

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
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

rust-info:
    rustc --version
    cargo --version

# Run all tests as expected by CI
ci-test: rust-info test-fmt clippy check test test-doc

# Run minimal subset of tests to ensure compatibility with MSRV
ci-test-msrv: rust-info check test

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

# Verify that the current version of the crate is not the same as the one published on crates.io
check-if-published:
    #!/usr/bin/env bash
    LOCAL_VERSION="$(grep '^version =' Cargo.toml | sed -E 's/version = "([^"]*)".*/\1/')"
    echo "Detected crate version:  $LOCAL_VERSION"
    CRATE_NAME="$(grep '^name =' Cargo.toml | head -1 | sed -E 's/name = "(.*)"/\1/')"
    echo "Detected crate name:     $CRATE_NAME"
    PUBLISHED_VERSION="$(cargo search ${CRATE_NAME} | grep "^${CRATE_NAME} =" | sed -E 's/.* = "(.*)".*/\1/')"
    echo "Published crate version: $PUBLISHED_VERSION"
    if [ "$LOCAL_VERSION" = "$PUBLISHED_VERSION" ]; then
        echo "ERROR: The current crate version has already been published."
        exit 1
    else
        echo "The current crate version has not yet been published."
    fi

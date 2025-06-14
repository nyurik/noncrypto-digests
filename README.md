# noncrypto-digests

[![GitHub repo](https://img.shields.io/badge/github-noncrypto--digests-8da0cb?logo=github)](https://github.com/nyurik/noncrypto-digests)
[![crates.io version](https://img.shields.io/crates/v/noncrypto-digests)](https://crates.io/crates/noncrypto-digests)
[![crate usage](https://img.shields.io/crates/d/noncrypto-digests)](https://crates.io/crates/noncrypto-digests)
[![docs.rs status](https://img.shields.io/docsrs/noncrypto-digests)](https://docs.rs/noncrypto-digests)
[![crates.io license](https://img.shields.io/crates/l/noncrypto-digests)](https://github.com/nyurik/noncrypto-digests/blob/main/LICENSE-APACHE)
[![CI build status](https://github.com/nyurik/noncrypto-digests/actions/workflows/ci.yml/badge.svg)](https://github.com/nyurik/noncrypto-digests/actions)
[![Codecov](https://img.shields.io/codecov/c/github/nyurik/noncrypto-digests)](https://app.codecov.io/gh/nyurik/noncrypto-digests)

Implement [digest::Digest](https://docs.rs/digest/latest/digest/trait.Digest.html) trait for non-cryptographic hashing functions like fnv and xxhash. This allows users to use all cryptographic and non-cryptographic hashing functions polymorphically.

## Usage

```rust
use digest::Digest;
use hex::ToHex;
use noncrypto_digests::{Fnv, Xxh3_64, Xxh3_128, Xxh32, Xxh64};

/// This function takes any Digest type, and returns a hex-encoded string.
pub fn hash<T: Digest>(data: impl AsRef<[u8]>) -> String {
    // Note that some hashers provide seed value set to 0 by default.
    // Use `...::from_hasher(hasher)` function to instantiate them.
    let mut hasher = T::new();
    hasher.update(data);
    hasher.finalize().to_vec().encode_hex_upper()
}

assert_eq!(hash::<Fnv>("password"), "4B1A493507B3A318");
assert_eq!(hash::<Xxh3_64>("password"), "336576D7E0E06F9A");
assert_eq!(hash::<Xxh3_128>("password"), "9CFA9055952177DA0B120BE86072A8F0");
assert_eq!(hash::<Xxh32>("password"), "106C6CED");
assert_eq!(hash::<Xxh64>("password"), "90007DAF3980EF1F");
```

## Development

* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`.
  Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.

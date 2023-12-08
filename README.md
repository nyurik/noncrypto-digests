# noncrypto-digests

[![GitHub](https://img.shields.io/badge/github-noncrypto--digests-8da0cb?logo=github)](https://github.com/nyurik/noncrypto-digests)
[![crates.io version](https://img.shields.io/crates/v/noncrypto-digests.svg)](https://crates.io/crates/noncrypto-digests)
[![docs.rs docs](https://docs.rs/noncrypto-digests/badge.svg)](https://docs.rs/noncrypto-digests)
[![crates.io version](https://img.shields.io/crates/l/noncrypto-digests.svg)](https://github.com/nyurik/noncrypto-digests/blob/main/LICENSE-APACHE)
[![CI build](https://github.com/nyurik/noncrypto-digests/actions/workflows/ci.yml/badge.svg)](https://github.com/nyurik/noncrypto-digests/actions)


Expose various non-cryptographic hashing functions with Digest traits.  This allows users to use any hashing function with the same trait interface, and switch between them easily.

## Usage

```rust
use digest::Digest;
use hex::ToHex;
use noncrypto_digests::Fnv;

/// This function takes any Digest type, and returns a hex-encoded string.
pub fn hash<T: Digest>(data: impl AsRef<[u8]>) -> String {
  let mut hasher = T::new();
  hasher.update(data);
  hasher.finalize().to_vec().encode_hex_upper()
}

fn main() {
  // Use Fnv hash
  assert_eq!(hash::<Fnv>("password"), "4B1A493507B3A318");
}
```

## Development
* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`. Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.
* On `git push`, it will run a few validations, including `cargo fmt`, `cargo clippy`, and `cargo test`.  Use `git push --no-verify` to skip these checks.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

//! Most importantly we need to make sure the hash object is accessible

use digest::Digest;
use hex::ToHex;
use insta::assert_snapshot;

pub fn hash<T: Digest>(data: impl AsRef<[u8]>) -> String {
    let mut hasher = T::new();
    hasher.update(data);
    hasher.finalize().to_vec().encode_hex_upper()
}

#[test]
#[cfg(feature = "fnv")]
fn fnv() {
    assert_snapshot!(hash::<noncrypto_digests::Fnv>("password"), @"4B1A493507B3A318");
}

#[test]
#[cfg(feature = "xxh3")]
fn xxh3() {
    assert_snapshot!(hash::<noncrypto_digests::Xxh3_64>("password"), @"336576D7E0E06F9A");
    assert_snapshot!(hash::<noncrypto_digests::Xxh3_128>("password"), @"9CFA9055952177DA0B120BE86072A8F0");
}

#[test]
#[cfg(feature = "xxh32")]
fn xxh32() {
    assert_snapshot!(hash::<noncrypto_digests::Xxh32>("password"), @"106C6CED");
}

#[test]
#[cfg(feature = "xxh64")]
fn xxh64() {
    assert_snapshot!(hash::<noncrypto_digests::Xxh64>("password"), @"90007DAF3980EF1F");
}

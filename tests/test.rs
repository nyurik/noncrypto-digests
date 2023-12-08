use digest::Digest;
use hex::ToHex;
use insta::assert_snapshot;

pub fn hash<T: Digest>(data: impl AsRef<[u8]>) -> String {
    let mut hasher = T::new();
    hasher.update(data);
    hasher.finalize().to_vec().encode_hex_upper()
}

/// Most importantly we need to make sure the hash object is accessible
#[test]
#[cfg(feature = "fnv")]
fn fnv() {
    assert_snapshot!(hash::<noncrypto_digests::Fnv>("password"), @"4B1A493507B3A318");
}

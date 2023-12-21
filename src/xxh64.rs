pub use ::xxhash_rust::xxh64::Xxh64 as Xxh64Hasher;
use digest::typenum::U8;
use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};

use crate::common::{impl_hash_wrapper, HashWrapper};

#[derive(Clone)]
pub struct Xxh64(Xxh64Hasher);

impl_hash_wrapper!(Xxh64, Xxh64Hasher, U8);

impl Default for Xxh64 {
    fn default() -> Self {
        Self(Xxh64Hasher::new(0))
    }
}

impl Update for Xxh64 {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl FixedOutput for Xxh64 {
    fn finalize_into(self, out: &mut Output<Self>) {
        let result = self.0.digest();
        let bytes = result.to_be_bytes();
        out.copy_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use xxhash_rust::xxh64::xxh64;

    use super::*;
    use crate::tests::hash;

    #[test]
    fn test_xxh64() {
        let default = xxh64(&[], 0);
        assert_eq!(hash::<Xxh64>(""), format!("{default:0>8X}"));
        assert_snapshot!(hash::<Xxh64>(""), @"EF46DB3751D8E999");
        assert_snapshot!(hash::<Xxh64>("hello"), @"26C7827D889F6DA3");
    }
}

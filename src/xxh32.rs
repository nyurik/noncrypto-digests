pub use ::xxhash_rust::xxh32::Xxh32 as Xxh32Hasher;
use digest::typenum::U4;
use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};

use crate::common::{impl_hash_wrapper, HashWrapper};

#[derive(Clone)]
pub struct Xxh32(Xxh32Hasher);

impl_hash_wrapper!(Xxh32, Xxh32Hasher, U4);

impl Default for Xxh32 {
    fn default() -> Self {
        Self(Xxh32Hasher::new(0))
    }
}

impl Update for Xxh32 {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl FixedOutput for Xxh32 {
    fn finalize_into(self, out: &mut Output<Self>) {
        let result = self.0.digest();
        let bytes = result.to_be_bytes();
        out.copy_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use xxhash_rust::xxh32::xxh32;

    use super::*;
    use crate::tests::hash;

    #[test]
    fn test_xxh32() {
        let default = xxh32(&[], 0);
        assert_eq!(hash::<Xxh32>(""), format!("{default:0>8X}"));
        assert_snapshot!(hash::<Xxh32>(""), @"02CC5D05");
        assert_snapshot!(hash::<Xxh32>("hello"), @"FB0077F9");
    }
}

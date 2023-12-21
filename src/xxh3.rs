use std::hash::Hasher as _;

pub use ::xxhash_rust::xxh3::Xxh3 as Xxh3Hasher;
use digest::typenum::{U16, U8};
use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};

use crate::common::{impl_hash_wrapper, HashWrapper};

macro_rules! make_hasher {
    ($hash_wrapper:ident, $hasher:ty, $digest:expr, $output_size:ident) => {
        #[derive(Clone, Default)]
        pub struct $hash_wrapper($hasher);

        impl_hash_wrapper!($hash_wrapper, $hasher, $output_size);

        impl Update for $hash_wrapper {
            fn update(&mut self, data: &[u8]) {
                self.0.write(data);
            }
        }

        impl FixedOutput for $hash_wrapper {
            fn finalize_into(self, out: &mut Output<Self>) {
                let result = $digest(&self.0);
                let bytes = result.to_be_bytes();
                out.copy_from_slice(&bytes);
            }
        }
    };
}

make_hasher!(Xxh3_64, Xxh3Hasher, Xxh3Hasher::digest, U8);
make_hasher!(Xxh3_128, Xxh3Hasher, Xxh3Hasher::digest128, U16);

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use xxhash_rust::xxh3::{xxh3_128, xxh3_64};

    use super::{Xxh3_128, Xxh3_64};
    use crate::tests::hash;

    #[test]
    fn test_xxh3_64() {
        let default = xxh3_64(&[]);
        assert_eq!(hash::<Xxh3_64>(""), format!("{default:X}"));
        assert_snapshot!(hash::<Xxh3_64>(""), @"2D06800538D394C2");
        assert_snapshot!(hash::<Xxh3_64>("hello"), @"9555E8555C62DCFD");
    }

    #[test]
    fn test_xxh3_128() {
        let default = xxh3_128(&[]);
        assert_eq!(hash::<Xxh3_128>(""), format!("{default:X}"));
        assert_snapshot!(hash::<Xxh3_128>(""), @"99AA06D3014798D86001C324468D497F");
        assert_snapshot!(hash::<Xxh3_128>("hello"), @"B5E9C1AD071B3E7FC779CFAA5E523818");
    }
}

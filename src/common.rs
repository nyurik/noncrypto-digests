pub trait HashWrapper {
    type Hasher;

    fn from_hasher(hasher: Self::Hasher) -> Self;

    fn get_hasher(&self) -> &Self::Hasher;

    fn get_hasher_mut(&mut self) -> &mut Self::Hasher;
}

macro_rules! impl_hash_wrapper {
    ($hash_wrapper:ident, $hasher:ty, $output_size:ident) => {
        impl HashWrapper for $hash_wrapper {
            type Hasher = $hasher;

            fn from_hasher(hasher: Self::Hasher) -> Self {
                Self(hasher)
            }

            fn get_hasher(&self) -> &Self::Hasher {
                &self.0
            }

            fn get_hasher_mut(&mut self) -> &mut Self::Hasher {
                &mut self.0
            }
        }

        impl OutputSizeUser for $hash_wrapper {
            type OutputSize = $output_size;
        }

        impl HashMarker for $hash_wrapper {}
    };
}

pub(crate) use impl_hash_wrapper;

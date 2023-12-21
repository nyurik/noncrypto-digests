#![cfg_attr(feature = "default", doc = include_str!("../README.md"))]
#![forbid(unsafe_code)]

#[cfg(not(any(
    feature = "fnv",
    feature = "xxh3",
    feature = "xxh32",
    feature = "xxh64"
)))]
compile_error!("At least one of these features must be enabled: fnv, xxh3, xxh32, xxh64");

mod common;

#[cfg(feature = "fnv")]
mod fnv;
#[cfg(feature = "fnv")]
pub use fnv::{Fnv, FnvHasher};

#[cfg(feature = "xxh3")]
mod xxh3;
#[cfg(feature = "xxh3")]
pub use xxh3::{Xxh3Hasher, Xxh3_128, Xxh3_64};

#[cfg(feature = "xxh32")]
mod xxh32;
#[cfg(feature = "xxh32")]
pub use xxh32::{Xxh32, Xxh32Hasher};

#[cfg(feature = "xxh64")]
mod xxh64;
#[cfg(feature = "xxh64")]
pub use xxh64::{Xxh64, Xxh64Hasher};

#[cfg(test)]
mod tests {
    use std::panic::{RefUnwindSafe, UnwindSafe};

    use digest::Digest;
    use hex::ToHex;

    pub fn hash<T: Digest + Clone + UnwindSafe + RefUnwindSafe + 'static>(
        data: impl AsRef<[u8]>,
    ) -> String {
        let mut hasher = T::new();
        hasher.update(data);
        hasher.finalize().to_vec().encode_hex_upper()
    }
}

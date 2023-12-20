#![cfg_attr(feature = "default", doc = include_str!("../README.md"))]
#![forbid(unsafe_code)]

#[cfg(not(any(feature = "fnv", feature = "xxh3")))]
compile_error!("At least one of the features `fnv` or `xxh3` must be enabled.");

#[cfg(feature = "fnv")]
mod fnv;

#[cfg(feature = "fnv")]
pub use fnv::Fnv;

#[cfg(feature = "xxh3")]
mod xxh3;

#[cfg(feature = "xxh3")]
pub use xxh3::{Xxh3_128, Xxh3_64};

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

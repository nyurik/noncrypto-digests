#![cfg_attr(feature = "default", doc = include_str!("../README.md"))]
#![forbid(unsafe_code)]

#[cfg(not(any(feature = "fnv")))]
compile_error!("At least one of the features `fnv` must be enabled.");

#[cfg(feature = "fnv")]
mod fnv;

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

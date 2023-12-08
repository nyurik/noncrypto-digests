use std::hash::Hasher as StdHasher;

use digest::{FixedOutput, HashMarker, Output, OutputSizeUser, Update};
use fnv::FnvHasher;

#[derive(Default)]
pub struct Fnv(FnvHasher);

impl OutputSizeUser for Fnv {
    type OutputSize = digest::typenum::U8;
}

impl HashMarker for Fnv {}

impl Clone for Fnv {
    fn clone(&self) -> Self {
        Self(FnvHasher::with_key(self.0.finish()))
    }
}

impl Update for Fnv {
    fn update(&mut self, data: &[u8]) {
        self.0.write(data);
    }
}

impl FixedOutput for Fnv {
    fn finalize_into(self, out: &mut Output<Self>) {
        let result = self.0.finish();
        let bytes = result.to_ne_bytes();
        out.copy_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::Fnv;
    use crate::tests::hash;

    #[test]
    fn test_fnv() {
        assert_snapshot!(hash::<Fnv>(""), @"25232284E49CF2CB");
        assert_snapshot!(hash::<Fnv>("hello"), @"0BBDAA8046D830A4");
    }
}

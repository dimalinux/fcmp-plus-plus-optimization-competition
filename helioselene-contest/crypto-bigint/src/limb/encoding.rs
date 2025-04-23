//! Limb encoding

use super::{Limb, Word};
use crate::Encoding;

impl Encoding for Limb {
    type Repr = [u8; 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Limb(Word::from_le_bytes(bytes))
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }
}

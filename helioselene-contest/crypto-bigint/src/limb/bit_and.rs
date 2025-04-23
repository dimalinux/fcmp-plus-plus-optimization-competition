//! Limb bit and operations.

use super::Limb;

impl Limb {
    /// Calculates `a & b`.
    #[inline(always)]
    pub const fn bitand(self, rhs: Self) -> Self {
        Limb(self.0 & rhs.0)
    }
}

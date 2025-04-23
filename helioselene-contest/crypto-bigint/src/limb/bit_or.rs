//! Limb bit or operations.

use super::Limb;

impl Limb {
    /// Calculates `a | b`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Limb(self.0 | rhs.0)
    }
}

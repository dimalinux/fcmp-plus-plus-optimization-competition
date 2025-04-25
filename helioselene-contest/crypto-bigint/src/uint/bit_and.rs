//! [`Uint`] bitwise and operations.

use super::Uint;
use crate::Limb;

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes bitwise `a & b`.
    #[inline(always)]
    pub const fn bitand(&self, rhs: &Self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].bitand(rhs.limbs[i]);
            i += 1;
        }

        Self { limbs }
    }
}

//! [`Uint`] bitwise not operations.

use super::Uint;
use crate::Limb;

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes bitwise `!a`.
    #[inline(always)]
    pub const fn not(&self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i].0 = !self.limbs[i].0;
            i += 1;
        }

        Self { limbs }
    }
}

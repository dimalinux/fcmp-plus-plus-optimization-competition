use crate::{Limb, Uint};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Calculate the number of bits needed to represent this number.
    pub const fn bits_vartime(&self) -> usize {
        let mut i = LIMBS - 1;
        while i > 0 && self.limbs[i].0 == 0 {
            i -= 1;
        }

        let limb = self.limbs[i];
        Limb::BITS * (i + 1) - limb.leading_zeros()
    }
}

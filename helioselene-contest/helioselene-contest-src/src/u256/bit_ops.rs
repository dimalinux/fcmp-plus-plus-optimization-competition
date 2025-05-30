use crate::u256::{ct_choice::CtChoice, primitives::WORD_BITS, U256};

impl U256 {
    #[inline(always)]
    pub(crate) const fn bitand_limb(&self, rhs: u64) -> Self {
        Self {
            limbs: [
                self.limbs[0] & rhs,
                self.limbs[1] & rhs,
                self.limbs[2] & rhs,
                self.limbs[3] & rhs,
            ],
        }
    }

    /// Computes `self >> 1` in constant-time, returning [`CtChoice::TRUTHY`] if the overflowing bit
    /// was set, and [`CtChoice::FALSE`] otherwise.
    pub(crate) const fn shr_1(&self) -> (Self, CtChoice) {
        let mut shifted_bits = [0; Self::LIMBS];
        let mut i = 0;
        while i < Self::LIMBS {
            shifted_bits[i] = self.limbs[i] >> 1;
            i += 1;
        }

        let mut carry_bits = [0; Self::LIMBS];
        let mut i = 0;
        while i < Self::LIMBS {
            carry_bits[i] = self.limbs[i] << (WORD_BITS - 1);
            i += 1;
        }

        let mut limbs = [0; Self::LIMBS];

        let mut i = 0;
        while i < (Self::LIMBS - 1) {
            limbs[i] = shifted_bits[i] | carry_bits[i + 1];
            i += 1;
        }
        limbs[Self::LIMBS - 1] = shifted_bits[Self::LIMBS - 1];

        debug_assert!(
            carry_bits[Self::LIMBS - 1] == 0
                || carry_bits[Self::LIMBS - 1] == (1 << (u64::BITS - 1))
        );
        (
            Self::new(limbs),
            CtChoice::from_lsb(carry_bits[0] >> (u64::BITS - 1)),
        )
    }
}

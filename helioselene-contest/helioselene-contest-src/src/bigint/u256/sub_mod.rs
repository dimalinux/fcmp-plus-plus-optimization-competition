//! [`Uint`] subtraction modulus operations.

use crate::bigint::U256;

impl U256 {
    /// Computes `self - rhs mod p`.
    ///
    /// Assumes `self - rhs` as unbounded signed integer is in `[-p, p)`.
    pub const fn sub_mod(&self, rhs: &Self, p: &Self) -> Self {
        let (out, borrow) = self.subtract_with_borrow(rhs, 0);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        let mask = Self::new([borrow; Self::LIMBS]);

        out.wrapping_add(&p.bitand(&mask))
    }

    /// Returns `(self..., carry) - (rhs...) mod (p...)`, where `carry <= 1`.
    /// Assumes `-(p...) <= (self..., carry) - (rhs...) < (p...)`.
    #[inline(always)]
    pub(crate) const fn sub_mod_with_carry(&self, carry: u64, rhs: &Self, p: &Self) -> Self {
        debug_assert!(carry <= 1);

        let (out, borrow) = self.subtract_with_borrow(rhs, 0);

        // The new `borrow = Word::MAX` iff `carry == 0` and `borrow == Word::MAX`.
        let borrow = (!carry.wrapping_neg()) & borrow;

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        let mask = Self::new([borrow; Self::LIMBS]);

        out.wrapping_add(&p.bitand(&mask))
    }
}

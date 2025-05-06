//! [`Uint`] subtraction modulus operations.

use crate::bigint::{uint::Limb, U256};

impl U256 {
    /// Computes `self - rhs mod p`.
    ///
    /// Assumes `self - rhs` as unbounded signed integer is in `[-p, p)`.
    pub const fn sub_mod(&self, rhs: &Self, p: &Self) -> Self {
        let (out, borrow) = self.sbb(rhs, Limb::ZERO);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        let mask = Self::from_words([borrow.0; Self::LIMBS]);

        out.wrapping_add(&p.bitand(&mask))
    }

    /// Returns `(self..., carry) - (rhs...) mod (p...)`, where `carry <= 1`.
    /// Assumes `-(p...) <= (self..., carry) - (rhs...) < (p...)`.
    #[inline(always)]
    pub(crate) const fn sub_mod_with_carry(&self, carry: Limb, rhs: &Self, p: &Self) -> Self {
        debug_assert!(carry.0 <= 1);

        let (out, borrow) = self.sbb(rhs, Limb::ZERO);

        // The new `borrow = Word::MAX` iff `carry == 0` and `borrow == Word::MAX`.
        let borrow = (!carry.0.wrapping_neg()) & borrow.0;

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        let mask = Self::from_words([borrow; Self::LIMBS]);

        out.wrapping_add(&p.bitand(&mask))
    }
}

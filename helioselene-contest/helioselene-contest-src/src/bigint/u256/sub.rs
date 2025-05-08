//! [`Uint`] addition operations.

use crate::bigint::{ct_choice::CtChoice, word, Word, U256};

impl U256 {
    /// Computes `a - (b + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub(crate) const fn sbb(&self, rhs: &Self, mut borrow: Word) -> (Self, Word) {
        let mut limbs = [0; Self::LIMBS];
        let mut i = 0;

        while i < Self::LIMBS {
            let (w, b) = word::sbb(self.limbs[i], rhs.limbs[i], borrow);
            limbs[i] = w;
            borrow = b;
            i += 1;
        }

        (Self { limbs }, borrow)
    }

    /// Perform saturating subtraction, returning `ZERO` on underflow.
    pub const fn saturating_sub(&self, rhs: &Self) -> Self {
        let (res, underflow) = self.sbb(rhs, 0);
        Self::ct_select(&res, &Self::ZERO, CtChoice::from_mask(underflow))
    }

    /// Perform wrapping subtraction, returning the truthy value as the second element of the tuple
    /// if an underflow has occurred.
    pub(crate) const fn conditional_wrapping_sub(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Self::ct_select(&Self::ZERO, rhs, choice);
        let (res, borrow) = self.sbb(&actual_rhs, 0);
        (res, CtChoice::from_mask(borrow))
    }
}

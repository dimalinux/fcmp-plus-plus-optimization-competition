//! [`Uint`] addition operations.

use super::Uint;
use crate::bigint::{ct_choice::CtChoice, uint::Limb};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `a - (b + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub(crate) const fn sbb(&self, rhs: &Self, mut borrow: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, b) = self.limbs[i].sbb(rhs.limbs[i], borrow);
            limbs[i] = w;
            borrow = b;
            i += 1;
        }

        (Self { limbs }, borrow)
    }

    /// Perform saturating subtraction, returning `ZERO` on underflow.
    pub const fn saturating_sub(&self, rhs: &Self) -> Self {
        let (res, underflow) = self.sbb(rhs, Limb::ZERO);
        Self::ct_select(&res, &Self::ZERO, CtChoice::from_mask(underflow.0))
    }

    /// Perform wrapping subtraction, discarding underflow and wrapping around
    /// the boundary of the type.
    pub const fn wrapping_sub(&self, rhs: &Self) -> Self {
        self.sbb(rhs, Limb::ZERO).0
    }

    /// Perform wrapping subtraction, returning the truthy value as the second element of the tuple
    /// if an underflow has occurred.
    pub(crate) const fn conditional_wrapping_sub(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Uint::ct_select(&Uint::ZERO, rhs, choice);
        let (res, borrow) = self.sbb(&actual_rhs, Limb::ZERO);
        (res, CtChoice::from_mask(borrow.0))
    }
}

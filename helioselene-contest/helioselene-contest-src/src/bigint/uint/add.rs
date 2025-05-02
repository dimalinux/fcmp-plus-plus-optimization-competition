//! [`Uint`] addition operations.

use crate::bigint::{
    ct_choice::CtChoice,
    uint::{Limb, Uint},
};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `a + b + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub(crate) const fn adc(&self, rhs: &Self, mut carry: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, c) = self.limbs[i].adc(rhs.limbs[i], carry);
            limbs[i] = w;
            carry = c;
            i += 1;
        }

        (Self { limbs }, carry)
    }

    /// Perform saturating addition, returning `MAX` on overflow.
    pub const fn saturating_add(&self, rhs: &Self) -> Self {
        let (res, overflow) = self.adc(rhs, Limb::ZERO);
        Self::ct_select(&res, &Self::MAX, CtChoice::from_lsb(overflow.0))
    }

    /// Perform wrapping addition, discarding overflow.
    pub const fn wrapping_add(&self, rhs: &Self) -> Self {
        self.adc(rhs, Limb::ZERO).0
    }

    /// Perform wrapping addition, returning the truthy value as the second element of the tuple
    /// if an overflow has occurred.
    pub(crate) const fn conditional_wrapping_add(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Uint::ct_select(&Uint::ZERO, rhs, choice);
        let (sum, carry) = self.adc(&actual_rhs, Limb::ZERO);
        (sum, CtChoice::from_lsb(carry.0))
    }
}

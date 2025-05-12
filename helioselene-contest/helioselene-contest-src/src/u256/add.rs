//! [`Uint`] addition operations.

use super::{ct_choice::CtChoice, word, U256};

impl U256 {
    /// Computes `a + b + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub(crate) const fn adc(&self, rhs: &Self, mut carry: u64) -> (Self, u64) {
        let mut limbs = [0; Self::LIMBS];
        let mut i = 0;

        while i < Self::LIMBS {
            let (w, c) = word::adc(self.limbs[i], rhs.limbs[i], carry);
            limbs[i] = w;
            carry = c;
            i += 1;
        }

        (Self { limbs }, carry)
    }

    /// Perform saturating addition, returning `MAX` on overflow.
    pub(crate) const fn saturating_add(&self, rhs: &Self) -> Self {
        let (res, overflow) = self.adc(rhs, 0);
        Self::ct_select(&res, &Self::MAX, CtChoice::from_lsb(overflow))
    }

    /// Perform wrapping addition, discarding overflow.
    pub(crate) const fn wrapping_add(&self, rhs: &Self) -> Self {
        self.adc(rhs, 0).0
    }

    /// Perform wrapping addition, returning the truthy value as the second element of the tuple
    /// if an overflow has occurred.
    pub(crate) const fn conditional_wrapping_add(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Self::ct_select(&Self::ZERO, rhs, choice);
        let (sum, carry) = self.adc(&actual_rhs, 0);
        (sum, CtChoice::from_lsb(carry))
    }
}

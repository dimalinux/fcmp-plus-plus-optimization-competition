//! [`Uint`] addition operations.

use super::{const_choice::ConstChoice, U256};
use crate::u256::primitives::carrying_add;

impl U256 {
    /// Computes `a + b + carry`, returning the result along with the new carry.
    #[inline]
    pub(crate) const fn carrying_add(&self, rhs: &Self, carry: u64) -> (Self, u64) {
        let (w0, carry) = carrying_add(self.limbs[0], rhs.limbs[0], carry);
        let (w1, carry) = carrying_add(self.limbs[1], rhs.limbs[1], carry);
        let (w2, carry) = carrying_add(self.limbs[2], rhs.limbs[2], carry);
        let (w3, carry) = carrying_add(self.limbs[3], rhs.limbs[3], carry);

        (
            Self {
                limbs: [w0, w1, w2, w3],
            },
            carry,
        )
    }

    /// Perform wrapping addition, discarding overflow.
    #[inline(always)]
    pub(crate) const fn wrapping_add(&self, rhs: &Self) -> Self {
        self.carrying_add(rhs, 0).0
    }

    /// Perform wrapping addition, returning the truthy value as the second element of the tuple
    /// if an overflow has occurred.
    pub(crate) const fn conditional_wrapping_add(
        &self,
        rhs: &Self,
        choice: ConstChoice,
    ) -> (Self, ConstChoice) {
        let actual_rhs = Self::ct_select(&Self::ZERO, rhs, choice);
        let (sum, carry) = self.carrying_add(&actual_rhs, 0);
        (sum, ConstChoice::from_lsb(carry))
    }
}

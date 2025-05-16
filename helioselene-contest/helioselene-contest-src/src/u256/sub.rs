//! [`Uint`] addition operations.

use crate::u256::{ct_choice::CtChoice, primitives::borrowing_sub, U256};

impl U256 {
    /// Computes `a - (b + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub(crate) const fn borrowing_sub(&self, rhs: &Self, borrow: u64) -> (Self, u64) {
        let (w0, borrow) = borrowing_sub(self.limbs[0], rhs.limbs[0], borrow);
        let (w1, borrow) = borrowing_sub(self.limbs[1], rhs.limbs[1], borrow);
        let (w2, borrow) = borrowing_sub(self.limbs[2], rhs.limbs[2], borrow);
        let (w3, borrow) = borrowing_sub(self.limbs[3], rhs.limbs[3], borrow);

        (
            Self {
                limbs: [w0, w1, w2, w3],
            },
            borrow,
        )
    }

    /// Perform wrapping subtraction, returning the truthy value as the second element of the tuple
    /// if an underflow has occurred.
    pub(crate) const fn conditional_wrapping_sub(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Self::ct_select(&Self::ZERO, rhs, choice);
        let (res, borrow) = self.borrowing_sub(&actual_rhs, 0);
        (res, CtChoice::from_mask(borrow))
    }
}

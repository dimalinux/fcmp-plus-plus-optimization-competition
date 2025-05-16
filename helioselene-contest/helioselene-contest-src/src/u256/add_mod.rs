//! [`Uint`] addition modulus operations.

use crate::u256::{primitives::borrowing_sub, U256};

impl U256 {
    /// Computes `self + rhs mod p`.
    ///
    /// Assumes `self + rhs` as unbounded integer is `< 2p`.
    pub(crate) const fn add_mod(&self, rhs: &Self, p: &Self) -> Self {
        let (w, carry) = self.carrying_add(rhs, 0);

        // Attempt to subtract the modulus, to ensure the result is in the field.
        let (w, borrow) = w.borrowing_sub(p, 0);
        let (_, borrow) = borrowing_sub(carry, 0, borrow);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
        // modulus.
        let mask = Self::new([borrow; Self::LIMBS]);

        w.wrapping_add(&p.bitand(&mask))
    }
}

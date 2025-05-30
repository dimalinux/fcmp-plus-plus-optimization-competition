//! Additions between integers in Montgomery form with a constant modulus.

use crate::u256::{primitives::borrowing_sub, MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Adds `rhs`.
    pub(crate) const fn add(&self, rhs: &Self) -> Self {
        let lhs = &self.montgomery_form;
        let rhs = &rhs.montgomery_form;

        let (res, carry) = lhs.carrying_add(rhs, 0);

        // Attempt to subtract the modulus, to ensure the result is in the field.
        let (mut res, borrow) = res.borrowing_sub(&MOD::MODULUS, 0);
        let (_, mask) = borrowing_sub(carry, 0, borrow);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
        // modulus.
        res = res.wrapping_add(&MOD::MODULUS.bitand_limb(mask));

        Self {
            montgomery_form: res,
            phantom: core::marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) const fn double(&self) -> Self {
        Self::add(self, self)
    }
}

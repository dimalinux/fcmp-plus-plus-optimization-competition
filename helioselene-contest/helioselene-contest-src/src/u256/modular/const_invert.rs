//! Multiplicative inverses of integers in Montgomery form with a constant modulus.

use core::marker::PhantomData;

use crate::u256::{ct_choice::CtChoice, MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Computes `self^-1` representing the multiplicative inverse of `self`,
    /// i.e. `self * self^-1 = 1`.
    ///
    /// If the number was invertible, the second element of the tuple is the truthy value,
    /// otherwise it is the falsy value (in which case the first element's value is unspecified).
    pub(crate) const fn invert(&self) -> (Self, CtChoice) {
        // Compute the inverse in Montgomery form.
        let (inverse, is_some) = self.montgomery_form.inv_odd_mod(&MOD::MODULUS);

        // Multiply with R3 and reduce in Montgomery form.
        let montgomery_form = Self::montgomery_reduction(&inverse.mul_wide(&MOD::R3));

        let value = Self {
            montgomery_form,
            phantom: PhantomData,
        };

        (value, is_some)
    }
}

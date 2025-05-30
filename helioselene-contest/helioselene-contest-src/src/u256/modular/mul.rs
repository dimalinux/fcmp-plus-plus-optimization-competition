//! Multiplications between integers in Montgomery form with a constant modulus.

use core::marker::PhantomData;

use crate::u256::{MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Multiplies by `rhs`.
    pub(crate) const fn mul(&self, rhs: &Self) -> Self {
        let product = self.montgomery_form.mul_wide(&rhs.montgomery_form);
        let montgomery_form = Self::montgomery_reduction(&product);
        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    /// Computes the (reduced) square.
    pub(crate) const fn square(&self) -> Self {
        let product = self.montgomery_form.square_wide();
        let montgomery_form = Self::montgomery_reduction(&product);
        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }
}

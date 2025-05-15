//! Negations of integers in Montgomery form with a constant modulus.

use core::ops::Neg;

use crate::u256::{MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Negates the number.
    pub(crate) const fn neg(&self) -> Self {
        Self::ZERO.sub(self)
    }
}

impl<MOD: MontyParams> Neg for MontyForm<MOD> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(&self)
    }
}

impl<MOD: MontyParams> Neg for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn neg(self) -> MontyForm<MOD> {
        MontyForm::neg(self)
    }
}

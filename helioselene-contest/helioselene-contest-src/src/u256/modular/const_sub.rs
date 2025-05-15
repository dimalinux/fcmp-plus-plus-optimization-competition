//! Subtractions between integers in Montgomery form with a constant modulus.

use core::ops::{Sub, SubAssign};

use super::{MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Subtracts `rhs`.
    pub(crate) const fn sub(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: self
                .montgomery_form
                .sub_mod(&rhs.montgomery_form, &MOD::MODULUS),
            phantom: core::marker::PhantomData,
        }
    }
}

impl<MOD: MontyParams> Sub<&MontyForm<MOD>> for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn sub(self, rhs: &MontyForm<MOD>) -> MontyForm<MOD> {
        MontyForm::sub(self, rhs)
    }
}

impl<MOD: MontyParams> Sub<MontyForm<MOD>> for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn sub(self, rhs: MontyForm<MOD>) -> MontyForm<MOD> {
        MontyForm::sub(self, &rhs)
    }
}

impl<MOD: MontyParams> Sub<&Self> for MontyForm<MOD> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self {
        Self::sub(&self, rhs)
    }
}

impl<MOD: MontyParams> Sub<Self> for MontyForm<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::sub(&self, &rhs)
    }
}

impl<MOD: MontyParams> SubAssign<&Self> for MontyForm<MOD> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Self::sub(self, rhs);
    }
}

impl<MOD: MontyParams> SubAssign<Self> for MontyForm<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(self, &rhs);
    }
}

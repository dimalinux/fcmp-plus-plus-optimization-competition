//! Multiplications between integers in Montgomery form with a constant modulus.

use core::{
    marker::PhantomData,
    ops::{Mul, MulAssign},
};

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

impl<MOD: MontyParams> Mul<&MontyForm<MOD>> for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn mul(self, rhs: &MontyForm<MOD>) -> MontyForm<MOD> {
        MontyForm::mul(self, rhs)
    }
}

impl<MOD: MontyParams> Mul<MontyForm<MOD>> for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn mul(self, rhs: MontyForm<MOD>) -> MontyForm<MOD> {
        MontyForm::mul(self, &rhs)
    }
}

impl<MOD: MontyParams> Mul<&Self> for MontyForm<MOD> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self {
        Self::mul(&self, rhs)
    }
}

impl<MOD: MontyParams> Mul<Self> for MontyForm<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::mul(&self, &rhs)
    }
}

impl<MOD: MontyParams> MulAssign<&Self> for MontyForm<MOD> {
    fn mul_assign(&mut self, rhs: &Self) {
        *self = Self::mul(self, rhs);
    }
}

impl<MOD: MontyParams> MulAssign<Self> for MontyForm<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::mul(self, &rhs);
    }
}

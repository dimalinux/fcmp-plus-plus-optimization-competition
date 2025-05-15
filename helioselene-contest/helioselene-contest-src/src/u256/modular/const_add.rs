//! Additions between integers in Montgomery form with a constant modulus.

use core::ops::{Add, AddAssign};

use crate::u256::{MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Adds `rhs`.
    #[inline]
    pub(crate) const fn add(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: self
                .montgomery_form
                .add_mod(&rhs.montgomery_form, &MOD::MODULUS),
            phantom: core::marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) const fn double(&self) -> Self {
        Self::add(self, self)
    }
}

impl<MOD: MontyParams> Add<&MontyForm<MOD>> for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn add(self, rhs: &MontyForm<MOD>) -> MontyForm<MOD> {
        MontyForm::add(self, rhs)
    }
}

impl<MOD: MontyParams> Add<MontyForm<MOD>> for &MontyForm<MOD> {
    type Output = MontyForm<MOD>;

    fn add(self, rhs: MontyForm<MOD>) -> MontyForm<MOD> {
        MontyForm::add(self, &rhs)
    }
}

impl<MOD: MontyParams> Add<&Self> for MontyForm<MOD> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self::add(&self, rhs)
    }
}

impl<MOD: MontyParams> Add<Self> for MontyForm<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::add(&self, &rhs)
    }
}

impl<MOD: MontyParams> AddAssign<&Self> for MontyForm<MOD> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self::add(self, rhs);
    }
}

impl<MOD: MontyParams> AddAssign<Self> for MontyForm<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(self, &rhs);
    }
}

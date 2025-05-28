//! Subtractions between integers in Montgomery form with a constant modulus.

use core::ops::{Sub, SubAssign};

use super::{MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Subtracts `rhs`.
    pub(crate) const fn sub(&self, rhs: &Self) -> Self {
        let lhs = &self.montgomery_form;
        let rhs = &rhs.montgomery_form;

        let (mut res, mask) = lhs.borrowing_sub(rhs, 0);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        res = res.wrapping_add(&MOD::MODULUS.bitand_limb(mask));

        Self {
            montgomery_form: res,
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

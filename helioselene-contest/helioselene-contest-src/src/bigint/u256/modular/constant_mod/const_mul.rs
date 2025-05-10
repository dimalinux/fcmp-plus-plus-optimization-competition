use core::{
    marker::PhantomData,
    ops::{Mul, MulAssign},
};

use super::{Residue, ResidueParams};
use crate::bigint::u256::modular::reduction::montgomery_reduction;

impl<MOD: ResidueParams> Residue<MOD> {
    /// Multiplies by `rhs`.
    pub(crate) const fn mul(&self, rhs: &Self) -> Self {
        let product = self.montgomery_form.mul_wide(&rhs.montgomery_form);
        let montgomery_form = montgomery_reduction(&product, &MOD::MODULUS, MOD::MOD_NEG_INV);
        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    /// Computes the (reduced) square of a residue.
    pub const fn square(&self) -> Self {
        let product = self.montgomery_form.square_wide();
        let montgomery_form = montgomery_reduction(&product, &MOD::MODULUS, MOD::MOD_NEG_INV);
        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }
}

impl<MOD: ResidueParams> Mul<&Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn mul(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        Residue::mul(self, rhs)
    }
}

impl<MOD: ResidueParams> Mul<Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn mul(self, rhs: Residue<MOD>) -> Residue<MOD> {
        Residue::mul(self, &rhs)
    }
}

impl<MOD: ResidueParams> Mul<&Residue<MOD>> for Residue<MOD> {
    type Output = Residue<MOD>;

    fn mul(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        Residue::mul(&self, rhs)
    }
}

impl<MOD: ResidueParams> Mul<Residue<MOD>> for Residue<MOD> {
    type Output = Residue<MOD>;

    fn mul(self, rhs: Residue<MOD>) -> Residue<MOD> {
        Residue::mul(&self, &rhs)
    }
}

impl<MOD: ResidueParams> MulAssign<&Self> for Residue<MOD> {
    fn mul_assign(&mut self, rhs: &Residue<MOD>) {
        *self = Residue::mul(self, rhs);
    }
}

impl<MOD: ResidueParams> MulAssign<Self> for Residue<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Residue::mul(self, &rhs);
    }
}

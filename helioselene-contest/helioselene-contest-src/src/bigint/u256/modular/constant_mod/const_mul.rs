use core::{
    marker::PhantomData,
    ops::{Mul, MulAssign},
};

use super::{Residue, ResidueParams};
use crate::bigint::u256::modular::mul::{mul_montgomery_form, square_montgomery_form};

impl<MOD: ResidueParams> Residue<MOD> {
    /// Multiplies by `rhs`.
    pub(crate) const fn mul(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: mul_montgomery_form(
                &self.montgomery_form,
                &rhs.montgomery_form,
                &MOD::MODULUS,
                MOD::MOD_NEG_INV,
            ),
            phantom: PhantomData,
        }
    }

    /// Computes the (reduced) square of a residue.
    pub const fn square(&self) -> Self {
        Self {
            montgomery_form: square_montgomery_form(
                &self.montgomery_form,
                &MOD::MODULUS,
                MOD::MOD_NEG_INV,
            ),
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

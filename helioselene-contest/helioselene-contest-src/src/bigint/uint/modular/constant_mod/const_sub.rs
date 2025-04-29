use core::ops::{Sub, SubAssign};

use super::{Residue, ResidueParams};
use crate::bigint::uint::modular::sub::sub_montgomery_form;

impl<MOD: ResidueParams> Residue<MOD> {
    /// Subtracts `rhs`.
    pub const fn sub(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: sub_montgomery_form(
                &self.montgomery_form,
                &rhs.montgomery_form,
                &MOD::MODULUS,
            ),
            phantom: core::marker::PhantomData,
        }
    }
}

impl<MOD: ResidueParams> Sub<&Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn sub(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        self.sub(rhs)
    }
}

impl<MOD: ResidueParams> Sub<Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    #[allow(clippy::op_ref)]
    fn sub(self, rhs: Residue<MOD>) -> Residue<MOD> {
        self - &rhs
    }
}

impl<MOD: ResidueParams> Sub<&Residue<MOD>> for Residue<MOD> {
    type Output = Residue<MOD>;

    #[allow(clippy::op_ref)]
    fn sub(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        &self - rhs
    }
}

impl<MOD: ResidueParams> Sub<Residue<MOD>> for Residue<MOD> {
    type Output = Residue<MOD>;

    fn sub(self, rhs: Residue<MOD>) -> Residue<MOD> {
        &self - &rhs
    }
}

impl<MOD: ResidueParams> SubAssign<&Self> for Residue<MOD> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = *self - rhs;
    }
}

impl<MOD: ResidueParams> SubAssign<Self> for Residue<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

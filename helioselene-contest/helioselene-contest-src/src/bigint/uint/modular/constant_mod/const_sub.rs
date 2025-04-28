use core::ops::{Sub, SubAssign};

use super::{Residue, ResidueParams};
use crate::bigint::uint::modular::sub::sub_montgomery_form;

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
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

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<&Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;

    fn sub(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self.sub(rhs)
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;

    #[allow(clippy::op_ref)]
    fn sub(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self - &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<&Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;

    #[allow(clippy::op_ref)]
    fn sub(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self - rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Sub<Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;

    fn sub(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self - &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> SubAssign<&Self> for Residue<MOD, LIMBS> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = *self - rhs;
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> SubAssign<Self> for Residue<MOD, LIMBS> {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

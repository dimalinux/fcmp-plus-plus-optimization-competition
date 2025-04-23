use core::ops::{Add, AddAssign};

use crate::modular::add::add_montgomery_form;

use super::{Residue, ResidueParams};

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
    /// Adds `rhs`.
    pub const fn add(&self, rhs: &Residue<MOD, LIMBS>) -> Self {
        Self {
            montgomery_form: add_montgomery_form(
                &self.montgomery_form,
                &rhs.montgomery_form,
                &MOD::MODULUS,
            ),
            phantom: core::marker::PhantomData,
        }
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Add<&Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    fn add(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self.add(rhs)
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Add<Residue<MOD, LIMBS>>
    for &Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    #[allow(clippy::op_ref)]
    fn add(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        self + &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Add<&Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    #[allow(clippy::op_ref)]
    fn add(self, rhs: &Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self + rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Add<Residue<MOD, LIMBS>>
    for Residue<MOD, LIMBS>
{
    type Output = Residue<MOD, LIMBS>;
    fn add(self, rhs: Residue<MOD, LIMBS>) -> Residue<MOD, LIMBS> {
        &self + &rhs
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> AddAssign<&Self> for Residue<MOD, LIMBS> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = *self + rhs;
    }
}

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> AddAssign<Self> for Residue<MOD, LIMBS> {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

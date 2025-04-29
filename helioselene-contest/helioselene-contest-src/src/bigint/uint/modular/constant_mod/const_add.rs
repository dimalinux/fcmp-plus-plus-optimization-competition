use core::ops::{Add, AddAssign};

use super::{Residue, ResidueParams};
use crate::bigint::uint::modular::add::add_montgomery_form;

impl<MOD: ResidueParams> Residue<MOD> {
    /// Adds `rhs`.
    pub const fn add(&self, rhs: &Residue<MOD>) -> Self {
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

impl<MOD: ResidueParams> Add<&Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn add(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        self.add(rhs)
    }
}

impl<MOD: ResidueParams> Add<Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    #[allow(clippy::op_ref)]
    fn add(self, rhs: Residue<MOD>) -> Residue<MOD> {
        self + &rhs
    }
}

impl<MOD: ResidueParams> Add<&Residue<MOD>> for Residue<MOD> {
    type Output = Residue<MOD>;

    #[allow(clippy::op_ref)]
    fn add(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        &self + rhs
    }
}

impl<MOD: ResidueParams> Add<Residue<MOD>> for Residue<MOD> {
    type Output = Residue<MOD>;

    fn add(self, rhs: Residue<MOD>) -> Residue<MOD> {
        &self + &rhs
    }
}

impl<MOD: ResidueParams> AddAssign<&Self> for Residue<MOD> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = *self + rhs;
    }
}

impl<MOD: ResidueParams> AddAssign<Self> for Residue<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

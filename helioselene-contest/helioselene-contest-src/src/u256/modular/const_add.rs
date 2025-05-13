use core::ops::{Add, AddAssign};

use crate::u256::{Residue, ResidueParams};

impl<MOD: ResidueParams> Residue<MOD> {
    /// Adds `rhs`.
    pub(crate) const fn add(&self, rhs: &Self) -> Self {
        Self {
            montgomery_form: self
                .montgomery_form
                .add_mod(&rhs.montgomery_form, &MOD::MODULUS),
            phantom: core::marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) const fn double(&self) -> Self {
        Self::add(self, self)
    }
}

impl<MOD: ResidueParams> Add<&Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn add(self, rhs: &Residue<MOD>) -> Residue<MOD> {
        Residue::add(self, rhs)
    }
}

impl<MOD: ResidueParams> Add<Residue<MOD>> for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn add(self, rhs: Residue<MOD>) -> Residue<MOD> {
        Residue::add(self, &rhs)
    }
}

impl<MOD: ResidueParams> Add<&Self> for Residue<MOD> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self::add(&self, rhs)
    }
}

impl<MOD: ResidueParams> Add<Self> for Residue<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::add(&self, &rhs)
    }
}

impl<MOD: ResidueParams> AddAssign<&Self> for Residue<MOD> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self::add(self, rhs);
    }
}

impl<MOD: ResidueParams> AddAssign<Self> for Residue<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(self, &rhs);
    }
}

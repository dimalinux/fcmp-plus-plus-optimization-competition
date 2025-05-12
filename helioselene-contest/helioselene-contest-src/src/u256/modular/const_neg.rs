use core::ops::Neg;

use crate::u256::{Residue, ResidueParams};

impl<MOD: ResidueParams> Residue<MOD> {
    /// Negates the number.
    pub(crate) const fn neg(&self) -> Self {
        Self::ZERO.sub(self)
    }
}

impl<MOD: ResidueParams> Neg for Residue<MOD> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::neg(&self)
    }
}

impl<MOD: ResidueParams> Neg for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn neg(self) -> Residue<MOD> {
        Residue::neg(self)
    }
}

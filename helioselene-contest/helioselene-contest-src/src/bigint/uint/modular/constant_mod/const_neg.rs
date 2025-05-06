use core::ops::Neg;

use super::{Residue, ResidueParams};

impl<MOD: ResidueParams> Residue<MOD> {
    /// Negates the number.
    pub const fn neg(&self) -> Self {
        Self::ZERO.sub(self)
    }
}

impl<MOD: ResidueParams> Neg for Residue<MOD> {
    type Output = Self;

    fn neg(self) -> Self {
        Residue::neg(&self)
    }
}

impl<MOD: ResidueParams> Neg for &Residue<MOD> {
    type Output = Residue<MOD>;

    fn neg(self) -> Residue<MOD> {
        Residue::neg(self)
    }
}

//! Negations of integers in Montgomery form with a constant modulus.

use crate::u256::{MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Negates the number.
    pub(crate) const fn neg(&self) -> Self {
        Self::ZERO.sub(self)
    }
}

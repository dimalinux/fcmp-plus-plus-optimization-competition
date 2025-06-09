//! Negations of integers in Montgomery form with a constant modulus.

use crate::u256::{CtChoice, MontyForm, MontyParams};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Negates the number.
    pub(crate) const fn neg(&self) -> Self {
        Self::ZERO.sub(self)
    }

    /// Conditionally negates the number
    pub(crate) const fn ct_neg(&self, choice: CtChoice) -> Self {
        Self::ct_select(self, &self.neg(), choice)
    }
}

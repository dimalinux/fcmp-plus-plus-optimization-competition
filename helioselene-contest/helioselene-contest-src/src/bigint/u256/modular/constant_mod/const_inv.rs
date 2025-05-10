use core::marker::PhantomData;

use super::{Residue, ResidueParams};
use crate::bigint::{ct_choice::CtChoice, u256::modular::reduction::montgomery_reduction};

impl<MOD: ResidueParams> Residue<MOD> {
    /// Computes the residue `self^-1` representing the multiplicative inverse of `self`.
    /// I.e. `self * self^-1 = 1`.
    /// If the number was invertible, the second element of the tuple is the truthy value,
    /// otherwise it is the falsy value (in which case the first element's value is unspecified).
    pub(crate) const fn invert(&self) -> (Self, CtChoice) {
        // Compute the inverse in Montgomery form.
        let (inverse, is_some) = self.montgomery_form.inv_odd_mod(&MOD::MODULUS);

        // Multiply with R3 and reduce in Montgomery form.
        let montgomery_form =
            montgomery_reduction(&inverse.mul_wide(&MOD::R3), &MOD::MODULUS, MOD::MOD_NEG_INV);

        let value = Self {
            montgomery_form,
            phantom: PhantomData,
        };

        (value, is_some)
    }
}

use core::marker::PhantomData;

use crate::{modular::inv::inv_montgomery_form, CtChoice};

use super::{Residue, ResidueParams};

impl<MOD: ResidueParams<LIMBS>, const LIMBS: usize> Residue<MOD, LIMBS> {
    /// Computes the residue `self^-1` representing the multiplicative inverse of `self`.
    /// I.e. `self * self^-1 = 1`.
    /// If the number was invertible, the second element of the tuple is the truthy value,
    /// otherwise it is the falsy value (in which case the first element's value is unspecified).
    pub const fn invert(&self) -> (Self, CtChoice) {
        let (montgomery_form, is_some) = inv_montgomery_form(
            &self.montgomery_form,
            &MOD::MODULUS,
            &MOD::R3,
            MOD::MOD_NEG_INV,
        );

        let value = Self {
            montgomery_form,
            phantom: PhantomData,
        };

        (value, is_some)
    }
}

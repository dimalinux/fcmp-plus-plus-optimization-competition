use super::{Residue, ResidueParams};
use crate::bigint::{u256::modular::pow::pow_montgomery_form, U256};

impl<MOD: ResidueParams> Residue<MOD> {
    /// Raises to the `exponent` power.
    pub const fn pow(&self, exponent: &U256) -> Residue<MOD> {
        self.pow_bounded_exp(exponent, U256::BITS)
    }

    /// Raises to the `exponent` power,
    /// with `exponent_bits` representing the number of (least significant) bits
    /// to take into account for the exponent.
    ///
    /// NOTE: `exponent_bits` may be leaked in the time pattern.
    const fn pow_bounded_exp(&self, exponent: &U256, exponent_bits: usize) -> Residue<MOD> {
        Self {
            montgomery_form: pow_montgomery_form(
                &self.montgomery_form,
                exponent,
                exponent_bits,
                &MOD::MODULUS,
                &MOD::R,
                MOD::MOD_NEG_INV,
            ),
            phantom: core::marker::PhantomData,
        }
    }
}

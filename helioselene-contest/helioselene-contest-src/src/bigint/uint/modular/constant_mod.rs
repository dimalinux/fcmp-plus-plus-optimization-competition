use core::{fmt::Debug, marker::PhantomData};

use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

use super::reduction::montgomery_reduction;
use crate::bigint::{
    uint::{Limb, Uint},
    Zero, U256,
};

/// Additions between residues with a constant modulus
mod const_add;
/// Multiplicative inverses of residues with a constant modulus
mod const_inv;
/// Multiplications between residues with a constant modulus
mod const_mul;
/// Negations of residues with a constant modulus
mod const_neg;
/// Exponentiation of residues with a constant modulus
mod const_pow;
/// Subtractions between residues with a constant modulus
mod const_sub;

//pub use macros::*;

/// The parameters to efficiently go to and from the Montgomery form for a given odd modulus.
///
/// Unfortunately, `LIMBS` must be generic for now until const generics are stabilized.
pub(crate) trait ResidueParams: Copy + Debug + Default + Eq + Send + Sync + 'static {
    /// The constant modulus
    const MODULUS: U256;
    /// 2^256 mod MODULUS, used to reduce 512-bit values
    const TWO_TO_256_MOD_M: U256;
    /// Parameter used in Montgomery reduction
    const R: U256;
    /// R^2, used to move into Montgomery form
    const R2: U256;
    /// R^3, used to perform a multiplicative inverse
    const R3: U256;
    /// The lowest limbs of -(MODULUS^-1) mod R
    // We only need the LSB because during reduction this value is multiplied modulo 2**Limb::BITS.
    const MOD_NEG_INV: Limb;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A residue mod `MOD`, represented using `LIMBS` limbs. The modulus of this residue is constant, so it cannot be set at runtime.
/// Internally, the value is stored in Montgomery form (multiplied by MOD::R) until it is retrieved.
pub(crate) struct Residue<MOD>
where
    MOD: ResidueParams,
{
    montgomery_form: U256,
    phantom: PhantomData<MOD>,
}

impl<MOD: ResidueParams> zeroize::DefaultIsZeroes for Residue<MOD> {}

impl<MOD: ResidueParams> Residue<MOD> {
    /// The representation of 1 mod `MOD`.
    pub(crate) const ONE: Self = Self {
        montgomery_form: MOD::R,
        phantom: PhantomData,
    };
    /// The representation of 0 mod `MOD`.
    pub(crate) const ZERO: Self = Self {
        montgomery_form: U256::ZERO,
        phantom: PhantomData,
    };

    // Internal helper function to generate a residue; this lets us wrap the constructors more cleanly
    const fn generate_residue(integer: &U256) -> Self {
        let product = integer.mul_wide(&MOD::R2);
        let montgomery_form = montgomery_reduction(&product, &MOD::MODULUS, MOD::MOD_NEG_INV);

        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    /// Instantiates a new `Residue` that represents this `integer` mod `MOD`.
    /// If the modulus represented by `MOD` is not odd, this function will panic; use [`new_checked`][`Residue::new_checked`] if you want to be able to detect an invalid modulus.
    pub(crate) const fn new(integer: &U256) -> Self {
        // TODO: make this check debug only?
        // A valid modulus must be odd
        if MOD::MODULUS.ct_is_odd().to_u8() == 0 {
            panic!("modulus must be odd");
        }

        Self::generate_residue(integer)
    }

    /// Instantiates a new `Residue` that represents this `integer` mod `MOD` if the modulus is odd.
    /// Returns a `CtOption` that is `None` if the provided modulus is not odd; this is a safer version of [`new`][`Residue::new`], which can panic.
    // TODO: remove this method when we can use `generic_const_exprs.` to ensure the modulus is
    // always valid.
    pub(crate) fn new_checked(integer: &U256) -> CtOption<Self> {
        // A valid modulus must be odd.
        CtOption::new(
            Self::generate_residue(integer),
            MOD::MODULUS.ct_is_odd().into(),
        )
    }

    /// Retrieves the integer currently encoded in this `Residue`, guaranteed to be reduced.
    pub(crate) const fn retrieve(&self) -> U256 {
        montgomery_reduction(
            &(self.montgomery_form, Uint::ZERO),
            &MOD::MODULUS,
            MOD::MOD_NEG_INV,
        )
    }
}

impl<MOD: ResidueParams + Copy> ConditionallySelectable for Residue<MOD> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Residue {
            montgomery_form: Uint::conditional_select(
                &a.montgomery_form,
                &b.montgomery_form,
                choice,
            ),
            phantom: PhantomData,
        }
    }
}

impl<MOD: ResidueParams> ConstantTimeEq for Residue<MOD> {
    fn ct_eq(&self, other: &Self) -> Choice {
        ConstantTimeEq::ct_eq(&self.montgomery_form, &other.montgomery_form)
    }
}

impl<MOD: ResidueParams> Default for Residue<MOD> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<MOD: ResidueParams> Zero for Residue<MOD> {
    const ZERO: Self = Self::ZERO;
}

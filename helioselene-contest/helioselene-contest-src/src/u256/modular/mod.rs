//! Implements `ConstMontyForm`s, supporting modular arithmetic with a constant modulus.
mod const_add;
mod const_invert;
mod const_mul;
mod const_neg;
mod const_sub;
pub(crate) mod pow;
mod reduction;

use core::{fmt::Debug, marker::PhantomData};

use crate::u256::{CtChoice, U256};

/// The parameters to efficiently go to and from the Montgomery form for a given odd modulus.
pub(crate) trait MontyParams: Copy + Debug + Default + Eq + Send + Sync + 'static {
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
    /// We only need the LSB because during reduction this value is multiplied modulo 2**WORD_BITS.
    const MOD_NEG_INV: u64;
}

/// An integer in Montgomery form modulo `MOD`. The modulus is constant, so it
/// cannot be set at runtime.
///
/// Internally, the value is stored in Montgomery form (multiplied by MOD::ONE)
/// until it is retrieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MontyForm<MOD: MontyParams> {
    montgomery_form: U256,
    phantom: PhantomData<MOD>,
}

impl<MOD: MontyParams> zeroize::DefaultIsZeroes for MontyForm<MOD> {}

impl<MOD: MontyParams> MontyForm<MOD> {
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

    /// Instantiates a new [`MontyForm`] that represents this `integer` mod `MOD`.
    pub(crate) const fn new(integer: &U256) -> Self {
        // TODO: make this check debug only?
        // A valid modulus must be odd
        assert!(MOD::MODULUS.ct_is_odd().to_u8() != 0, "modulus must be odd");

        let product = integer.mul_wide(&MOD::R2);
        let montgomery_form = Self::montgomery_reduction(&product);

        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    /// Convert the number back from the optimized representation.
    pub(crate) const fn retrieve(&self) -> U256 {
        Self::montgomery_reduction(&(self.montgomery_form, U256::ZERO))
    }
}

impl<MOD: MontyParams + Copy> MontyForm<MOD> {
    pub(crate) const fn ct_select(a: &Self, b: &Self, choice: CtChoice) -> Self {
        Self {
            montgomery_form: U256::ct_select(&a.montgomery_form, &b.montgomery_form, choice),
            phantom: PhantomData,
        }
    }
}

impl<MOD: MontyParams> MontyForm<MOD> {
    #[inline]
    pub(crate) const fn ct_eq(&self, other: &Self) -> CtChoice {
        U256::ct_eq(&self.montgomery_form, &other.montgomery_form)
    }

    #[inline]
    pub(crate) const fn ct_is_zero(&self) -> CtChoice {
        self.montgomery_form.ct_is_zero()
    }
}

impl<MOD: MontyParams> Default for MontyForm<MOD> {
    fn default() -> Self {
        Self::ZERO
    }
}

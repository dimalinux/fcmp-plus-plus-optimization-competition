//! Implements `ConstMontyForm`s, supporting modular arithmetic with a constant modulus.
mod add;
mod invert;
mod mul;
mod neg;
pub(crate) mod pow;
mod random;
mod reduction;
mod sqrt;
mod sub;

use core::{fmt::Debug, marker::PhantomData};

use crate::u256::{CtChoice, U256};

/// The parameters to efficiently go to and from the Montgomery form for a given odd modulus.
pub(crate) trait MontyParams: Copy + Debug + Default + Eq + Send + Sync + 'static {
    /// The constant modulus
    const MODULUS: U256;

    /// 256 minus the number of leading zero bits in the modulus.
    const MODULUS_BITS: usize;

    /// Parameter used in Montgomery reduction
    const R: U256;

    /// R^2, used to move into Montgomery form
    const R2: U256;

    /// R^3, used to perform a multiplicative inverse
    const R3: U256;

    /// MOD_3_8 is (MODULUS + 3) // 8, used for calculating square roots.
    /// Only used when MODULUS (p) satisfies p mod 8 = 5, in which case
    /// its value *must* be overridden.
    const MOD_3_8: U256 = U256::ZERO; // Must not be zero if p mod 8 = 5

    /// SQRT_M1 is 2^((MODULUS - 1) // 4) % MODULUS.
    /// Only used when MODULUS (p) satisfies p mod 8 = 5, in which case
    /// its value *must* be overridden.
    const SQRT_M1: MontyForm<Self> = MontyForm::ZERO;

    /// MOD_PLUS_1_DIV_4 is (MODULUS+1) // 4. Used for sqrt.
    /// Only used when MODULUS (p) satisfies p mod 4 = 3, in which case
    /// its value *must* be overridden.
    const MOD_PLUS_1_DIV_4: U256 = U256::ZERO;

    /// The lowest limbs of -(MODULUS^-1) mod R
    /// We only need the LSB because during reduction this value is multiplied modulo 2**WORD_BITS.
    const MOD_NEG_INV: u64;

    /// 2^256 mod MODULUS, used to reduce 512-bit values
    const TWO_TO_256_MOD_M: U256;
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
        // A valid modulus must be odd
        debug_assert!(MOD::MODULUS.ct_is_odd().is_true_vartime());

        let product = Self::mul_wide(integer, &MOD::R2);
        let montgomery_form = Self::montgomery_reduction(&product);

        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    pub(crate) const fn from_le_bytes(bytes: [u8; 32]) -> (Self, CtChoice) {
        let integer = U256::from_le_bytes(bytes);
        let less_than_modulus = U256::ct_lt(&integer, &MOD::MODULUS);
        (Self::new(&integer), less_than_modulus)
    }

    pub(crate) const fn to_le_bytes(self) -> [u8; 32] {
        self.retrieve().to_le_bytes()
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

//! Stack-allocated big unsigned integers.

#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]

mod add;
mod add_mod;
mod bit_and;
mod bit_not;
mod bit_or;
mod bits;
mod cmp;
mod concat;
mod div;
mod encoding;
mod from;
mod inv_mod;
mod mul;
mod neg;
mod shl;
mod shr;
mod sub;
mod sub_mod;

/// Implements modular arithmetic for constant moduli.
pub mod modular;

use crate::{traits, Encoding, Integer, Limb, Word, Zero};
use core::fmt;
use subtle::{Choice, ConditionallySelectable};
use zeroize::DefaultIsZeroes;

/// Stack-allocated big unsigned integer.
///
/// Generic over the given number of `LIMBS`
///
/// # Encoding support
/// This type supports many different types of encodings, either via the
/// [`Encoding`][`crate::Encoding`] trait or various `const fn` decoding and
/// encoding functions that can be used with [`Uint`] constants.
///
/// Optional crate features for encoding (off-by-default):
/// - `generic-array`: enables [`ArrayEncoding`][`crate::ArrayEncoding`] trait which can be used to
///   [`Uint`] as `GenericArray<u8, N>` and a [`ArrayDecoding`][`crate::ArrayDecoding`] trait which
///   can be used to `GenericArray<u8, N>` as [`Uint`].
/// - `rlp`: support for [Recursive Length Prefix (RLP)][RLP] encoding.
///
/// [RLP]: https://eth.wiki/fundamentals/rlp
// TODO(tarcieri): make generic around a specified number of bits.
// Our PartialEq impl only differs from the default one by being constant-time, so this is safe
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Copy, Clone, Hash)]
pub struct Uint<const LIMBS: usize> {
    /// Inner limb array. Stored from least significant to most significant.
    limbs: [Limb; LIMBS],
}

impl<const LIMBS: usize> Uint<LIMBS> {
    /// The value `0`.
    pub const ZERO: Self = Self::from_u8(0);

    /// The value `1`.
    pub const ONE: Self = Self::from_u8(1);

    /// Maximum value this [`Uint`] can express.
    pub const MAX: Self = Self {
        limbs: [Limb::MAX; LIMBS],
    };

    /// Total size of the represented integer in bits.
    pub const BITS: usize = LIMBS * Limb::BITS;

    /// Total size of the represented integer in bytes.
    pub const BYTES: usize = LIMBS * Limb::BYTES;

    /// The number of limbs used on this platform.
    pub const LIMBS: usize = LIMBS;

    /// Const-friendly [`Uint`] constructor.
    pub const fn new(limbs: [Limb; LIMBS]) -> Self {
        Self { limbs }
    }

    /// Create a [`Uint`] from an array of [`Word`]s (i.e. word-sized unsigned
    /// integers).
    #[inline]
    pub const fn from_words(arr: [Word; LIMBS]) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = Limb(arr[i]);
            i += 1;
        }

        Self { limbs }
    }

    /// Create an array of [`Word`]s (i.e. word-sized unsigned integers) from
    /// a [`Uint`].
    #[inline]
    pub const fn to_words(self) -> [Word; LIMBS] {
        let mut arr = [0; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            arr[i] = self.limbs[i].0;
            i += 1;
        }

        arr
    }

    /// Borrow the inner limbs as an array of [`Word`]s.
    pub const fn as_words(&self) -> &[Word; LIMBS] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &*((&self.limbs as *const _) as *const [Word; LIMBS])
        }
    }

    /// Borrow the inner limbs as a mutable array of [`Word`]s.
    pub fn as_words_mut(&mut self) -> &mut [Word; LIMBS] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &mut *((&mut self.limbs as *mut _) as *mut [Word; LIMBS])
        }
    }

    /// Borrow the limbs of this [`Uint`].
    pub const fn as_limbs(&self) -> &[Limb; LIMBS] {
        &self.limbs
    }

    /// Borrow the limbs of this [`Uint`] mutably.
    pub fn as_limbs_mut(&mut self) -> &mut [Limb; LIMBS] {
        &mut self.limbs
    }

    /// Convert this [`Uint`] into its inner limbs.
    pub const fn to_limbs(self) -> [Limb; LIMBS] {
        self.limbs
    }
}

impl<const LIMBS: usize> AsRef<[Word; LIMBS]> for Uint<LIMBS> {
    fn as_ref(&self) -> &[Word; LIMBS] {
        self.as_words()
    }
}

impl<const LIMBS: usize> AsMut<[Word; LIMBS]> for Uint<LIMBS> {
    fn as_mut(&mut self) -> &mut [Word; LIMBS] {
        self.as_words_mut()
    }
}

impl<const LIMBS: usize> AsRef<[Limb]> for Uint<LIMBS> {
    fn as_ref(&self) -> &[Limb] {
        self.as_limbs()
    }
}

impl<const LIMBS: usize> AsMut<[Limb]> for Uint<LIMBS> {
    fn as_mut(&mut self) -> &mut [Limb] {
        self.as_limbs_mut()
    }
}

impl<const LIMBS: usize> ConditionallySelectable for Uint<LIMBS> {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];

        for i in 0..LIMBS {
            limbs[i] = Limb::conditional_select(&a.limbs[i], &b.limbs[i], choice);
        }

        Self { limbs }
    }
}

impl<const LIMBS: usize> Default for Uint<LIMBS> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<const LIMBS: usize> Integer for Uint<LIMBS> {
    const ONE: Self = Self::ONE;
    const MAX: Self = Self::MAX;
    const BITS: usize = Self::BITS;
    const BYTES: usize = Self::BYTES;
    const LIMBS: usize = Self::LIMBS;

    fn is_odd(&self) -> Choice {
        self.limbs
            .first()
            .map(|limb| limb.is_odd())
            .unwrap_or_else(|| Choice::from(0))
    }
}

impl<const LIMBS: usize> Zero for Uint<LIMBS> {
    const ZERO: Self = Self::ZERO;
}

impl<const LIMBS: usize> fmt::Debug for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uint(0x{self:X})")
    }
}

impl<const LIMBS: usize> fmt::Display for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self, f)
    }
}

impl<const LIMBS: usize> fmt::LowerHex for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            fmt::LowerHex::fmt(limb, f)?;
        }
        Ok(())
    }
}

impl<const LIMBS: usize> fmt::UpperHex for Uint<LIMBS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            fmt::UpperHex::fmt(limb, f)?;
        }
        Ok(())
    }
}

impl<const LIMBS: usize> DefaultIsZeroes for Uint<LIMBS> {}

#[doc = "256-bit"]
#[doc = "unsigned big integer."]
pub type U256 = Uint<{ 256 / Limb::BITS }>;
impl Encoding for U256 {
    type Repr = [u8; 256 / 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Self::from_le_slice(&bytes)
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        let mut result = [0u8; 256 / 8];
        self.write_le_bytes(&mut result);
        result
    }
}

#[doc = "512-bit"]
#[doc = "unsigned big integer."]
pub type U512 = Uint<{ 512 / Limb::BITS }>;
impl Encoding for U512 {
    type Repr = [u8; 512 / 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Self::from_le_slice(&bytes)
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        let mut result = [0u8; 512 / 8];
        self.write_le_bytes(&mut result);
        result
    }
}

// Implement concat and split for double-width Uint sizes: these should be
// multiples of 128 bits.
impl traits::ConcatMixed<Uint<{ <U256>::LIMBS / 2 }>> for Uint<{ <U256>::LIMBS / 2 }>
{
    type MixedOutput = U256;

    fn concat_mixed(&self, lo: &Uint<{ <U256>::LIMBS / 2 }>) -> Self::MixedOutput {
        concat::concat_mixed(lo, self)
    }
}
impl Uint<{ <U256>::LIMBS / 2 }> {
    ///   Concatenate the two values, with  `self`  as most significant and  `rhs`
    ///   as the least significant.
    pub const fn concat(&self, lo: &Uint<{ <U256>::LIMBS / 2 }>) -> U256 {
        concat::concat_mixed(lo, self)
    }
}

impl traits::ConcatMixed<Uint<{ <U512>::LIMBS / 2 }>> for Uint<{ <U512>::LIMBS / 2 }>
{
    type MixedOutput = U512;

    fn concat_mixed(&self, lo: &Uint<{ <U512>::LIMBS / 2 }>) -> Self::MixedOutput {
        concat::concat_mixed(lo, self)
    }
}
impl Uint<{ <U512>::LIMBS / 2 }> {
    ///   Concatenate the two values, with  `self`  as most significant and  `rhs`
    ///   as the least significant.
    pub const fn concat(&self, lo: &Uint<{ <U512>::LIMBS / 2 }>) -> U512 {
        concat::concat_mixed(lo, self)
    }
}

impl U512 {
    /// Creates a `U512` from a tuple of two `U256` values.
    ///
    /// # Parameters
    /// - `nums`: A tuple `(low, high)` where:
    ///   - `low` is the least significant `U256`.
    ///   - `high` is the most significant `U256`.
    ///
    /// # Returns
    /// A `U512` value constructed by combining the `low` and `high` parts.
    pub fn from(nums: (U256, U256)) -> Self {
        let mut to = Self::ZERO;
        to.limbs[..<U256>::LIMBS].copy_from_slice(nums.0.as_limbs());
        to.limbs[<U256>::LIMBS..].copy_from_slice(nums.1.as_limbs());
        to
    }
}

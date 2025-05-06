//! Stack-allocated big unsigned integers.

#![allow(clippy::needless_range_loop, clippy::many_single_char_names)]

mod add;
mod add_mod;
mod bit_ops;
mod cmp;
mod div;
mod encoding;
mod from;
mod inv_mod;
mod mul;
mod neg;
mod sub;
mod sub_mod;

/// Implements modular arithmetic for constant moduli.
pub(crate) mod modular;

use core::fmt;

use subtle::{Choice, ConditionallySelectable};
use zeroize::DefaultIsZeroes;

use crate::bigint::{
    limb::{Limb, Word},
    Encoding, Zero,
};

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
pub struct U256 {
    /// Inner limb array. Stored from least significant to most significant.
    limbs: [Limb; U256::LIMBS],
}

impl U256 {
    /// Total size of the represented integer in bits.
    pub(crate) const BITS: usize = 256;
    /// Total size of the represented integer in bytes.
    pub(crate) const BYTES: usize = Self::BITS / 8;
    /// The number of limbs used on this platform.
    pub(crate) const LIMBS: usize = Self::BITS / Limb::BITS;
    /// Maximum value this [`Uint`] can express.
    pub(crate) const MAX: Self = Self {
        limbs: [Limb::MAX; Self::LIMBS],
    };
    /// The value `1`.
    pub(crate) const ONE: Self = Self::from_u8(1);
    /// The value `0`.
    pub(crate) const ZERO: Self = Self::from_u8(0);

    /// Const-friendly [`Uint`] constructor.
    pub(crate) const fn new(limbs: [Limb; Self::LIMBS]) -> Self {
        Self { limbs }
    }

    /// Create a [`Uint`] from an array of [`Word`]s (i.e. word-sized unsigned
    /// integers).
    #[inline]
    pub(crate) const fn from_words(arr: [Word; Self::LIMBS]) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];
        let mut i = 0;

        while i < Self::LIMBS {
            limbs[i] = Limb(arr[i]);
            i += 1;
        }

        Self { limbs }
    }

    /// Borrow the inner limbs as an array of [`Word`]s.
    pub(crate) const fn as_words(&self) -> &[Word; Self::LIMBS] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &*((&self.limbs as *const _) as *const [Word; Self::LIMBS])
        }
    }

    /// Borrow the inner limbs as a mutable array of [`Word`]s.
    pub(crate) fn as_words_mut(&mut self) -> &mut [Word; Self::LIMBS] {
        // SAFETY: `Limb` is a `repr(transparent)` newtype for `Word`
        #[allow(trivial_casts, unsafe_code)]
        unsafe {
            &mut *((&mut self.limbs as *mut _) as *mut [Word; Self::LIMBS])
        }
    }

    /// Borrow the limbs of this [`Uint`].
    pub(crate) const fn as_limbs(&self) -> &[Limb; Self::LIMBS] {
        &self.limbs
    }

    pub(crate) fn is_odd(&self) -> Choice {
        self.limbs
            .first()
            .map(|limb| limb.is_odd())
            .unwrap_or_else(|| Choice::from(0))
    }
}

impl AsRef<[Word; U256::LIMBS]> for U256 {
    fn as_ref(&self) -> &[Word; Self::LIMBS] {
        self.as_words()
    }
}

impl AsMut<[Word; U256::LIMBS]> for U256 {
    fn as_mut(&mut self) -> &mut [Word; Self::LIMBS] {
        self.as_words_mut()
    }
}

impl AsRef<[Limb]> for U256 {
    fn as_ref(&self) -> &[Limb] {
        &self.limbs
    }
}

impl AsMut<[Limb]> for U256 {
    fn as_mut(&mut self) -> &mut [Limb] {
        &mut self.limbs
    }
}

impl ConditionallySelectable for U256 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];

        for i in 0..Self::LIMBS {
            limbs[i] = Limb::conditional_select(&a.limbs[i], &b.limbs[i], choice);
        }

        Self { limbs }
    }
}

impl Default for U256 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Zero for U256 {
    const ZERO: Self = Self::ZERO;
}

impl fmt::Debug for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Uint(0x{self:X})")
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(self, f)
    }
}

impl fmt::LowerHex for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            write!(f, "{:0width$x}", limb.0, width = Self::BYTES * 2)?;
        }
        Ok(())
    }
}

impl fmt::UpperHex for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            write!(f, "{:0width$X}", limb.0, width = Self::BYTES * 2)?;
        }
        Ok(())
    }
}

impl DefaultIsZeroes for U256 {}

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

    #[inline]
    #[cfg(test)]
    fn to_be_bytes(&self) -> Self::Repr {
        let mut result = [0u8; 256 / 8];
        self.write_be_bytes(&mut result);
        result
    }
}

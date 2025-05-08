//! Stack-allocated 256-bit integer.
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

use crate::bigint::{Encoding, Zero};

const WORD_BITS: usize = u64::BITS as usize;
const WORD_BYTES: usize = WORD_BITS / 8;

/// Wide integer type: double the width of [`crate::bigint::u64`].
//pub(crate) type WideWord = u128;

/// Stack-allocated 256-bit unsigned integer.
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Copy, Clone, Hash)]
pub struct U256 {
    /// Inner limb array. Stored from least significant to most significant.
    limbs: [u64; 4],
}

impl U256 {
    /// Total size of the represented integer in bits.
    pub(crate) const BITS: usize = 256;
    /// Total size of the represented integer in bytes.
    pub(crate) const BYTES: usize = 32;
    // 256 bits / 8 bits per byte
    /// The number of limbs used on this platform.
    pub(crate) const LIMBS: usize = 4;
    // 4 u64 limbs = 256 bits
    /// Maximum value this [`Uint`] can express.
    pub(crate) const MAX: Self = Self {
        limbs: [u64::MAX; Self::LIMBS],
    };
    /// The value `1`.
    pub(crate) const ONE: Self = Self::from_u64(1);
    /// The value `0`.
    pub(crate) const ZERO: Self = Self::from_u64(0);

    // TODO: use default?

    /// Const [`Uint`] constructor from an array of [`u64`]s.
    #[inline]
    pub(crate) const fn new(limbs: [u64; Self::LIMBS]) -> Self {
        Self { limbs }
    }

    /// Borrow the limbs of this [`Uint`].
    pub(crate) const fn as_words(&self) -> &[u64; Self::LIMBS] {
        &self.limbs
    }

    pub(crate) fn is_odd(&self) -> Choice {
        Choice::from((self.limbs[0] & 1) as u8)
    }
}

impl ConditionallySelectable for U256 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        let mut limbs = [0; Self::LIMBS];

        for i in 0..Self::LIMBS {
            limbs[i] = u64::conditional_select(&a.limbs[i], &b.limbs[i], choice);
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
        write!(f, "Uint(0x{self:x})")
    }
}

impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::LowerHex for U256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            write!(f, "{:0width$x}", limb, width = Self::BYTES * 2)?;
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

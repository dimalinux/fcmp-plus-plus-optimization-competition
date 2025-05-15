mod add;
mod add_mod;
mod bit_ops;
mod cmp;
/// Implements modular arithmetic for constant moduli.
mod ct_choice;
mod encoding;
mod from;
mod inv_mod;
mod modular;
mod mul;
mod neg;
mod sub;
mod sub_mod;
mod traits;
mod word;

use ::core::fmt;
pub(crate) use modular::{MontyForm, MontyParams};
use subtle::{Choice, ConditionallySelectable};
pub(crate) use traits::{Encoding, Zero};
use zeroize::DefaultIsZeroes;

/// Stack-allocated 256-bit unsigned integer.
#[derive(Default, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) struct U256 {
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
    #[cfg(test)]
    pub(crate) const MAX: Self = Self {
        limbs: [u64::MAX; Self::LIMBS],
    };
    /// The value `1`.
    pub(crate) const ONE: Self = Self::from_u64(1);
    /// The value `0`.
    pub(crate) const ZERO: Self = Self::from_u64(0);

    /// Const [`Uint`] constructor from an array of [`u64`]s.
    #[inline]
    pub(crate) const fn new(limbs: [u64; Self::LIMBS]) -> Self {
        Self { limbs }
    }

    pub(crate) fn is_odd(&self) -> Choice {
        Choice::from((self.limbs[0] & 1) as u8)
    }
}

impl ConditionallySelectable for U256 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            limbs: [
                u64::conditional_select(&a.limbs[0], &b.limbs[0], choice),
                u64::conditional_select(&a.limbs[1], &b.limbs[1], choice),
                u64::conditional_select(&a.limbs[2], &b.limbs[2], choice),
                u64::conditional_select(&a.limbs[3], &b.limbs[3], choice),
            ],
        }
    }
}

impl Zero for U256 {
    const ZERO: Self = Self::ZERO;
}

impl DefaultIsZeroes for U256 {}

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

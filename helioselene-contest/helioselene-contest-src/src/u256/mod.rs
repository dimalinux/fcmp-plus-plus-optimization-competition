mod add;
mod bit_ops;
mod cmp;
mod ct_choice;
mod encoding;
mod from;
mod inv_mod;
mod modular;
mod mul;
mod neg;
mod primitives;
mod sub;
mod traits;

use ::core::fmt;
pub(crate) use modular::{MontyForm, MontyParams};
pub(crate) use traits::Encoding;
use zeroize::DefaultIsZeroes;

pub(crate) use crate::u256::ct_choice::CtChoice;

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
    /// The number of limbs used on this platform.
    pub(crate) const LIMBS: usize = 4;
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

    #[inline]
    pub(crate) const fn least_significant_bit(&self) -> u64 {
        self.limbs[0] & 1
    }

    #[inline]
    pub(crate) const fn is_odd(&self) -> CtChoice {
        CtChoice::from_lsb(self.limbs[0] & 1)
    }
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

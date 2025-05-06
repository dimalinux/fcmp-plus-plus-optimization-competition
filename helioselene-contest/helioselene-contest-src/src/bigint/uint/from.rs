//! `From`-like conversions for [`U256`].

use crate::bigint::{
    uint::{Limb, Word},
    U256,
};

impl U256 {
    /// Create a [`Uint`] from a `u8` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u8>` when stable
    pub const fn from_u8(n: u8) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u16` (const-friendly)
    // TODO: replace with `const impl From<u16>` when stable
    pub const fn from_u16(n: u16) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u32` (const-friendly)
    // TODO: replace with `const impl From<u32>` when stable
    #[allow(trivial_numeric_casts)]
    pub const fn from_u32(n: u32) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u64` (const-friendly)
    // TODO: replace with `const impl From<u64>` when stable
    pub const fn from_u64(n: u64) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u128` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u128>` when stable
    pub const fn from_u128(n: u128) -> Self {
        let lo = n as u64;
        let hi = (n >> 64) as u64;

        let mut limbs = [Limb::ZERO; Self::LIMBS];

        limbs[0].0 = lo;
        limbs[1].0 = hi;

        Self { limbs }
    }

    /// Create a [`Uint`] from a `Word` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<Word>` when stable
    pub const fn from_word(n: Word) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }
}

impl From<u8> for U256 {
    fn from(n: u8) -> Self {
        Self::from_u8(n)
    }
}

impl From<u16> for U256 {
    fn from(n: u16) -> Self {
        Self::from_u16(n)
    }
}

impl From<u32> for U256 {
    fn from(n: u32) -> Self {
        Self::from_u32(n)
    }
}

impl From<u64> for U256 {
    fn from(n: u64) -> Self {
        Self::from_u64(n)
    }
}

impl From<u128> for U256 {
    fn from(n: u128) -> Self {
        Self::from_u128(n)
    }
}

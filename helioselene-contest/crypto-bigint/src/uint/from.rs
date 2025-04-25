//! `From`-like conversions for [`Uint`].

use crate::{Limb, Uint, WideWord, Word};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Create a [`Uint`] from a `u8` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u8>` when stable
    pub const fn from_u8(n: u8) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u16` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u16>` when stable
    pub const fn from_u16(n: u16) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u32` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u32>` when stable
    #[allow(trivial_numeric_casts)]
    pub const fn from_u32(n: u32) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u64` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u64>` when stable
    pub const fn from_u64(n: u64) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `u128` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u128>` when stable
    pub const fn from_u128(n: u128) -> Self {
        assert!(
            LIMBS >= (128 / Limb::BITS),
            "number of limbs must be greater than zero"
        );

        let lo = n as u64;
        let hi = (n >> 64) as u64;

        let mut limbs = [Limb::ZERO; LIMBS];

        limbs[0].0 = lo;
        limbs[1].0 = hi;

        Self { limbs }
    }

    /// Create a [`Uint`] from a `Word` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<Word>` when stable
    pub const fn from_word(n: Word) -> Self {
        assert!(LIMBS >= 1, "number of limbs must be greater than zero");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n;
        Self { limbs }
    }

    /// Create a [`Uint`] from a `WideWord` (const-friendly)
    // TODO(tarcieri): replace with `const impl From<WideWord>` when stable
    pub const fn from_wide_word(n: WideWord) -> Self {
        assert!(LIMBS >= 2, "number of limbs must be two or greater");
        let mut limbs = [Limb::ZERO; LIMBS];
        limbs[0].0 = n as Word;
        limbs[1].0 = (n >> Limb::BITS) as Word;
        Self { limbs }
    }
}

impl<const LIMBS: usize> From<u8> for Uint<LIMBS> {
    fn from(n: u8) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS > 0, "limbs must be non-zero");
        Self::from_u8(n)
    }
}

impl<const LIMBS: usize> From<u16> for Uint<LIMBS> {
    fn from(n: u16) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS > 0, "limbs must be non-zero");
        Self::from_u16(n)
    }
}

impl<const LIMBS: usize> From<u32> for Uint<LIMBS> {
    fn from(n: u32) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS > 0, "limbs must be non-zero");
        Self::from_u32(n)
    }
}

impl<const LIMBS: usize> From<u64> for Uint<LIMBS> {
    fn from(n: u64) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS >= (64 / Limb::BITS), "not enough limbs");
        Self::from_u64(n)
    }
}

impl<const LIMBS: usize> From<u128> for Uint<LIMBS> {
    fn from(n: u128) -> Self {
        // TODO(tarcieri): const where clause when possible
        debug_assert!(LIMBS >= (128 / Limb::BITS), "not enough limbs");
        Self::from_u128(n)
    }
}

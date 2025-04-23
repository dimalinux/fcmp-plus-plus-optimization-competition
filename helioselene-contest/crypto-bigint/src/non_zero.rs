//! Wrapper type for non-zero integers.

use crate::{CtChoice, Encoding, Integer, Limb, Uint, Zero};
use core::{
    fmt,
    num::{NonZeroU128, NonZeroU64, NonZeroU8},
    ops::Deref,
};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};

/// Wrapper type for non-zero integers.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct NonZero<T: Zero>(T);

impl NonZero<Limb> {
    /// Creates a new non-zero limb in a const context.
    /// The second return value is `FALSE` if `n` is zero, `TRUE` otherwise.
    pub const fn const_new(n: Limb) -> (Self, CtChoice) {
        (Self(n), n.ct_is_nonzero())
    }
}

impl<const LIMBS: usize> NonZero<Uint<LIMBS>> {
    /// Creates a new non-zero integer in a const context.
    /// The second return value is `FALSE` if `n` is zero, `TRUE` otherwise.
    pub const fn const_new(n: Uint<LIMBS>) -> (Self, CtChoice) {
        (Self(n), n.ct_is_nonzero())
    }
}

impl<T> NonZero<T>
where
    T: Zero,
{
    /// Create a new non-zero integer.
    pub fn new(n: T) -> CtOption<Self> {
        let is_zero = n.is_zero();
        CtOption::new(Self(n), !is_zero)
    }
}

impl<T> NonZero<T>
where
    T: Integer,
{
    /// The value `1`.
    pub const ONE: Self = Self(T::ONE);

    /// Maximum value this integer can express.
    pub const MAX: Self = Self(T::MAX);
}

impl<T> NonZero<T>
where
    T: Encoding + Zero,
{
    /// Decode from little endian bytes.
    pub fn from_le_bytes(bytes: T::Repr) -> CtOption<Self> {
        Self::new(T::from_le_bytes(bytes))
    }
}

impl<T> AsRef<T> for NonZero<T>
where
    T: Zero,
{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ConditionallySelectable for NonZero<T>
where
    T: ConditionallySelectable + Zero,
{
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(T::conditional_select(&a.0, &b.0, choice))
    }
}

impl<T> ConstantTimeEq for NonZero<T>
where
    T: Zero,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl<T> Deref for NonZero<T>
where
    T: Zero,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> fmt::Display for NonZero<T>
where
    T: fmt::Display + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<T> fmt::Binary for NonZero<T>
where
    T: fmt::Binary + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl<T> fmt::Octal for NonZero<T>
where
    T: fmt::Octal + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

impl<T> fmt::LowerHex for NonZero<T>
where
    T: fmt::LowerHex + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<T> fmt::UpperHex for NonZero<T>
where
    T: fmt::UpperHex + Zero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl NonZero<Limb> {
    /// Create a [`NonZero<Limb>`] from a [`NonZeroU8`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU8>` when stable
    pub const fn from_u8(n: NonZeroU8) -> Self {
        Self(Limb::from_u8(n.get()))
    }

    /// Create a [`NonZero<Limb>`] from a [`NonZeroU64`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU64>` when stable
    pub const fn from_u64(n: NonZeroU64) -> Self {
        Self(Limb::from_u64(n.get()))
    }
}

impl From<NonZeroU8> for NonZero<Limb> {
    fn from(integer: NonZeroU8) -> Self {
        Self::from_u8(integer)
    }
}

impl From<NonZeroU64> for NonZero<Limb> {
    fn from(integer: NonZeroU64) -> Self {
        Self::from_u64(integer)
    }
}

impl<const LIMBS: usize> NonZero<Uint<LIMBS>> {
    /// Create a [`NonZero<Uint>`] from a [`Uint`] (const-friendly)
    pub const fn from_uint(n: Uint<LIMBS>) -> Self {
        let mut i = 0;
        let mut found_non_zero = false;
        while i < LIMBS {
            if n.as_limbs()[i].0 != 0 {
                found_non_zero = true;
            }
            i += 1;
        }
        assert!(found_non_zero, "found zero");
        Self(n)
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU8`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU8>` when stable
    pub const fn from_u8(n: NonZeroU8) -> Self {
        Self(Uint::from_u8(n.get()))
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU64`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU64>` when stable
    pub const fn from_u64(n: NonZeroU64) -> Self {
        Self(Uint::from_u64(n.get()))
    }

    /// Create a [`NonZero<Uint>`] from a [`NonZeroU128`] (const-friendly)
    // TODO(tarcieri): replace with `const impl From<NonZeroU128>` when stable
    pub const fn from_u128(n: NonZeroU128) -> Self {
        Self(Uint::from_u128(n.get()))
    }
}

impl<const LIMBS: usize> From<NonZeroU8> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU8) -> Self {
        Self::from_u8(integer)
    }
}

impl<const LIMBS: usize> From<NonZeroU64> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU64) -> Self {
        Self::from_u64(integer)
    }
}

impl<const LIMBS: usize> From<NonZeroU128> for NonZero<Uint<LIMBS>> {
    fn from(integer: NonZeroU128) -> Self {
        Self::from_u128(integer)
    }
}

//! Wrapper type for non-zero integers.

use crate::{CtChoice, Encoding, Integer, Limb, Uint, Zero};
use core::ops::Deref;
use subtle::CtOption;

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

impl<T> Deref for NonZero<T>
where
    T: Zero,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

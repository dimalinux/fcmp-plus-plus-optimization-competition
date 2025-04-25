//! Traits provided by this crate

#[allow(unused_imports)]
use crate::{Limb, NonZero};
use core::fmt::Debug;
use subtle::{
    Choice, ConstantTimeEq
};

/// Integer type.
pub trait Integer:
    'static
    + AsRef<[Limb]>
    + Copy
    + Debug
    + Default
    + Eq
    + From<u64>
    + Sized
    + Zero
{
    /// The value `1`.
    const ONE: Self;

    /// Maximum value this integer can express.
    const MAX: Self;

    /// Total size of the represented integer in bits.
    const BITS: usize;

    /// Total size of the represented integer in bytes.
    const BYTES: usize;

    /// The number of limbs used on this platform.
    const LIMBS: usize;

    /// Is this integer value an odd number?
    ///
    /// # Returns
    ///
    /// If odd, returns `Choice(1)`. Otherwise, returns `Choice(0)`.
    fn is_odd(&self) -> Choice;

    /// Is this integer value an even number?
    ///
    /// # Returns
    ///
    /// If even, returns `Choice(1)`. Otherwise, returns `Choice(0)`.
    fn is_even(&self) -> Choice {
        !self.is_odd()
    }
}

/// Zero values.
pub trait Zero: ConstantTimeEq + Sized {
    /// The value `0`.
    const ZERO: Self;

    /// Determine if this value is equal to zero.
    ///
    /// # Returns
    ///
    /// If zero, returns `Choice(1)`. Otherwise, returns `Choice(0)`.
    fn is_zero(&self) -> Choice {
        self.ct_eq(&Self::ZERO)
    }
}

/// Compute `self + rhs mod p`.
pub trait AddMod<Rhs = Self> {
    /// Output type.
    type Output;

    /// Compute `self + rhs mod p`.
    ///
    /// Assumes `self` and `rhs` are `< p`.
    fn add_mod(&self, rhs: &Rhs, p: &Self) -> Self::Output;
}

/// Concatenate two numbers into a "wide" double-width value, using the `lo`
/// value as the least significant value.
pub trait Concat: ConcatMixed<Self, MixedOutput = Self::Output> {
    /// Concatenated output: twice the width of `Self`.
    type Output;

    /// Concatenate the two halves, with `self` as most significant and `lo`
    /// as the least significant.
    fn concat(&self, lo: &Self) -> Self::Output {
        self.concat_mixed(lo)
    }
}

/// Concatenate two numbers into a "wide" combined-width value, using the `lo`
/// value as the least significant value.
pub trait ConcatMixed<Lo: ?Sized = Self> {
    /// Concatenated output: combination of `Lo` and `Self`.
    type MixedOutput;

    /// Concatenate the two values, with `self` as most significant and `lo`
    /// as the least significant.
    fn concat_mixed(&self, lo: &Lo) -> Self::MixedOutput;
}

/// Encoding support.
pub trait Encoding: Sized {
    /// Byte array representation.
    type Repr: AsRef<[u8]> + AsMut<[u8]> + Copy + Clone + Sized;

    /// Decode from little endian bytes.
    fn from_le_bytes(bytes: Self::Repr) -> Self;

    /// Encode to little endian bytes.
    fn to_le_bytes(&self) -> Self::Repr;
}

/// Support for optimized squaring
pub trait Square: Sized
where
    for<'a> &'a Self: core::ops::Mul<&'a Self, Output = Self>,
{
    /// Computes the same as `self.mul(self)`, but may be more efficient.
    fn square(&self) -> Self {
        self * self
    }
}

/// Constant-time exponentiation.
pub trait Pow<Exponent> {
    /// Raises to the `exponent` power.
    fn pow(&self, exponent: &Exponent) -> Self;
}

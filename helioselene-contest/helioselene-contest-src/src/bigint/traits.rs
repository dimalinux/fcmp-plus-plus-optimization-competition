//! Traits provided by this crate

use core::fmt::Debug;

use subtle::{Choice, ConstantTimeEq};

use crate::bigint::limb::Limb;

/// Integer type.
pub(crate) trait Integer:
    'static + AsRef<[Limb]> + Copy + Debug + Default + Eq + From<u64> + Sized + Zero
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
}

/// Zero values.
pub(crate) trait Zero: ConstantTimeEq + Sized {
    /// The value `0`.
    const ZERO: Self;
}

/// Encoding support.
pub(crate) trait Encoding: Sized {
    /// Byte array representation.
    type Repr: AsRef<[u8]> + AsMut<[u8]> + Copy + Clone + Sized;

    /// Decode from little endian bytes.
    fn from_le_bytes(bytes: Self::Repr) -> Self;

    /// Encode to little endian bytes.
    fn to_le_bytes(&self) -> Self::Repr;

    /// Encode to bit endian bytes.
    #[cfg(test)]
    fn to_be_bytes(&self) -> Self::Repr;
}

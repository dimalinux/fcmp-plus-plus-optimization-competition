//! Big integers are represented as an array of smaller CPU word-size integers
//! called "limbs".

use core::cmp::Ordering;

use subtle::{
    Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess,
};

use crate::bigint::{ct_choice::CtChoice, Encoding, Zero};

/// Unsigned integer type that the [`Limb`] newtype wraps.
pub type Word = u64;

/// Wide integer type: double the width of [`Word`].
pub type WideWord = u128;

/// Highest bit in a [`Limb`].
pub(crate) const HI_BIT: usize = Limb::BITS - 1;

/// Big integers are represented as an array of smaller CPU word-size integers
/// called "limbs".
// Our PartialEq impl only differs from the default one by being constant-time, so this is safe
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Copy, Clone, Default, Hash)]
#[repr(transparent)]
pub struct Limb(pub Word);

impl Limb {
    /// Size of the inner integer in bits.
    pub const BITS: usize = 64;
    /// Size of the inner integer in bytes.
    pub const BYTES: usize = 8;
    /// Maximum value this [`Limb`] can express.
    pub const MAX: Self = Limb(Word::MAX);
    /// The value `1`.
    pub const ONE: Self = Limb(1);
    /// The value `0`.
    pub const ZERO: Self = Limb(0);
}
impl ConditionallySelectable for Limb {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(Word::conditional_select(&a.0, &b.0, choice))
    }
}

impl Zero for Limb {
    const ZERO: Self = Self::ZERO;
}

impl zeroize::DefaultIsZeroes for Limb {}

impl Limb {
    /// Computes `self + rhs + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn adc(self, rhs: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = rhs.0 as WideWord;
        let carry = carry.0 as WideWord;
        let ret = a + b + carry;
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }

    /// Computes `self - (rhs + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub const fn sbb(self, rhs: Limb, borrow: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = rhs.0 as WideWord;
        let borrow = (borrow.0 >> (Self::BITS - 1)) as WideWord;
        let ret = a.wrapping_sub(b + borrow);
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }

    /// Computes `self + (b * c) + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn mac(self, b: Limb, c: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = b.0 as WideWord;
        let c = c.0 as WideWord;
        let carry = carry.0 as WideWord;
        let ret = a + (b * c) + carry;
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }

    /// Calculates `a & b`.
    #[inline(always)]
    pub const fn bitand(self, rhs: Self) -> Self {
        Limb(self.0 & rhs.0)
    }

    /// Calculates `a | b`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Limb(self.0 | rhs.0)
    }

    /// Calculate the number of bits needed to represent this number.
    pub const fn bits(self) -> usize {
        Limb::BITS - (self.0.leading_zeros() as usize)
    }

    /// Calculate the number of leading zeros in the binary representation of this number.
    pub const fn leading_zeros(self) -> usize {
        self.0.leading_zeros() as usize
    }

    /// Is this limb an odd number?
    #[inline]
    pub fn is_odd(&self) -> Choice {
        Choice::from(self.0 as u8 & 1)
    }

    /// Perform a comparison of the inner value in variable-time.
    ///
    /// Note that the [`PartialOrd`] and [`Ord`] impls wrap constant-time
    /// comparisons using the `subtle` crate.
    pub fn cmp_vartime(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    /// Performs an equality check in variable-time.
    pub const fn eq_vartime(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Return `b` if `c` is truthy, otherwise return `a`.
    #[inline]
    pub(crate) const fn ct_select(a: Self, b: Self, c: CtChoice) -> Self {
        Self(c.select(a.0, b.0))
    }

    /// Returns the truthy value if `self != 0` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_is_nonzero(&self) -> CtChoice {
        let inner = self.0;
        CtChoice::from_lsb((inner | inner.wrapping_neg()) >> HI_BIT)
    }

    /// Returns the truthy value if `lhs == rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_eq(lhs: Self, rhs: Self) -> CtChoice {
        let x = lhs.0;
        let y = rhs.0;

        // x ^ y == 0 if and only if x == y
        Self(x ^ y).ct_is_nonzero().not()
    }
}

impl ConstantTimeEq for Limb {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl ConstantTimeGreater for Limb {
    #[inline]
    fn ct_gt(&self, other: &Self) -> Choice {
        self.0.ct_gt(&other.0)
    }
}

impl ConstantTimeLess for Limb {
    #[inline]
    fn ct_lt(&self, other: &Self) -> Choice {
        self.0.ct_lt(&other.0)
    }
}

impl Eq for Limb {}

impl Ord for Limb {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut n = 0i8;
        n -= self.ct_lt(other).unwrap_u8() as i8;
        n += self.ct_gt(other).unwrap_u8() as i8;

        match n {
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => {
                debug_assert_eq!(n, 0);
                debug_assert!(bool::from(self.ct_eq(other)));
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Limb {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Limb {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl Encoding for Limb {
    type Repr = [u8; 8];

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Limb(Word::from_le_bytes(bytes))
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }
}

impl Limb {
    /// Create a [`Limb`] from a `u8` integer (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u8>` when stable
    pub const fn from_u8(n: u8) -> Self {
        Limb(n as Word)
    }

    /// Create a [`Limb`] from a `u16` integer (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u16>` when stable
    pub const fn from_u16(n: u16) -> Self {
        Limb(n as Word)
    }

    /// Create a [`Limb`] from a `u32` integer (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u32>` when stable
    pub const fn from_u32(n: u32) -> Self {
        #[allow(trivial_numeric_casts)]
        Limb(n as Word)
    }

    /// Create a [`Limb`] from a `u64` integer (const-friendly)
    // TODO(tarcieri): replace with `const impl From<u64>` when stable
    pub const fn from_u64(n: u64) -> Self {
        Limb(n)
    }
}

impl From<u8> for Limb {
    #[inline]
    fn from(n: u8) -> Limb {
        Limb(n.into())
    }
}

impl From<u16> for Limb {
    #[inline]
    fn from(n: u16) -> Limb {
        Limb(n.into())
    }
}

impl From<u32> for Limb {
    #[inline]
    fn from(n: u32) -> Limb {
        Limb(n.into())
    }
}

impl From<u64> for Limb {
    #[inline]
    fn from(n: u64) -> Limb {
        Limb(n)
    }
}

impl From<Limb> for Word {
    #[inline]
    fn from(limb: Limb) -> Word {
        limb.0
    }
}

impl From<Limb> for WideWord {
    #[inline]
    fn from(limb: Limb) -> WideWord {
        limb.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_zero() {
        assert!(bool::from(Limb::ZERO.is_zero()));
        assert!(!bool::from(Limb::ONE.is_zero()));
        assert!(!bool::from(Limb::MAX.is_zero()));
    }

    #[test]
    fn is_odd() {
        assert!(!bool::from(Limb::ZERO.is_odd()));
        assert!(bool::from(Limb::ONE.is_odd()));
        assert!(bool::from(Limb::MAX.is_odd()));
    }

    #[test]
    fn ct_eq() {
        let a = Limb::ZERO;
        let b = Limb::MAX;

        assert!(bool::from(a.ct_eq(&a)));
        assert!(!bool::from(a.ct_eq(&b)));
        assert!(!bool::from(b.ct_eq(&a)));
        assert!(bool::from(b.ct_eq(&b)));
    }

    #[test]
    fn ct_gt() {
        let a = Limb::ZERO;
        let b = Limb::ONE;
        let c = Limb::MAX;

        assert!(bool::from(b.ct_gt(&a)));
        assert!(bool::from(c.ct_gt(&a)));
        assert!(bool::from(c.ct_gt(&b)));

        assert!(!bool::from(a.ct_gt(&a)));
        assert!(!bool::from(b.ct_gt(&b)));
        assert!(!bool::from(c.ct_gt(&c)));

        assert!(!bool::from(a.ct_gt(&b)));
        assert!(!bool::from(a.ct_gt(&c)));
        assert!(!bool::from(b.ct_gt(&c)));
    }

    #[test]
    fn ct_lt() {
        let a = Limb::ZERO;
        let b = Limb::ONE;
        let c = Limb::MAX;

        assert!(bool::from(a.ct_lt(&b)));
        assert!(bool::from(a.ct_lt(&c)));
        assert!(bool::from(b.ct_lt(&c)));

        assert!(!bool::from(a.ct_lt(&a)));
        assert!(!bool::from(b.ct_lt(&b)));
        assert!(!bool::from(c.ct_lt(&c)));

        assert!(!bool::from(b.ct_lt(&a)));
        assert!(!bool::from(c.ct_lt(&a)));
        assert!(!bool::from(c.ct_lt(&b)));
    }

    #[test]
    fn cmp() {
        assert_eq!(Limb::ZERO.cmp(&Limb::ONE), Ordering::Less);
        assert_eq!(Limb::ONE.cmp(&Limb::ONE), Ordering::Equal);
        assert_eq!(Limb::MAX.cmp(&Limb::ONE), Ordering::Greater);
    }

    #[test]
    fn adc_no_carry() {
        let (res, carry) = Limb::ZERO.adc(Limb::ONE, Limb::ZERO);
        assert_eq!(res.0, 1);
        assert_eq!(carry.0, 0);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = Limb::MAX.adc(Limb::ONE, Limb::ZERO);
        assert_eq!(res.0, 0);
        assert_eq!(carry.0, 1);
    }

    #[test]
    fn sbb_no_borrow() {
        let (res, borrow) = Limb::ONE.sbb(Limb::ONE, Limb::ZERO);
        assert_eq!(res.0, 0);
        assert_eq!(borrow.0, 0);
    }

    #[test]
    fn sbb_with_borrow() {
        let (res, borrow) = Limb::ZERO.sbb(Limb::ONE, Limb::ZERO);

        assert_eq!(res.0, Limb::MAX.0);
        assert_eq!(borrow.0, Limb::MAX.0);
    }
}

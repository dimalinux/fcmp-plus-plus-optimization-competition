use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use group::ff::{Field, FieldBits, PrimeField, PrimeFieldBits};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::u256::{CtChoice, FixedExponent, MontyForm, MontyParams, U256};

const MODULUS_HEX: &str = "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct Field25519Params;

impl MontyParams for Field25519Params {
    /// MODULUS is 2^255 - 19 (an odd value)
    const MODULUS: U256 = U256::from_be_hex(MODULUS_HEX);
    /// MODULUS_BITS is the number of bits in the modulus (ours has one leading zero bit)
    const MODULUS_BITS: usize = 255;
    /// MOD_3_8 is (MODULUS + 3) // 8, used for calculating square roots
    const MOD_3_8: U256 =
        U256::from_be_hex("0ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe");
    /// MOD_NEG_INV is the modular multiplicative inverse of the least
    /// significant 64-bits of `MODULUS` modulo 2^64, negated.
    const MOD_NEG_INV: u64 = 0x86bca1af286bca1b_u64;
    /// R is U256::MAX % MODULUS + 1
    const R: U256 = U256::from_u64(0x26);
    /// R2 is R^2 mod MODULUS
    const R2: U256 = U256::from_u64(0x5a4);
    /// R3 is the montgomery form of R2^2
    const R3: U256 = U256::from_u64(0xd658);
    /// SQRT_M1 is 2^((MODULUS - 1) // 4) % MODULUS.
    const SQRT_M1: MontyForm<Self> = MontyForm::new(&U256::from_be_hex(
        "2b8324804fc1df0b2b4d00993dfbd7a72f431806ad2fe478c4ee1b274a0ea0b0",
    ));
    /// TWO_TO_256_MOD_M is 2^256 mod MODULUS
    const TWO_TO_256_MOD_M: U256 = U256::from_u64(0x26);
}

type MontyFormType = MontyForm<Field25519Params>;

/// A constant-time implementation of the Ed25519 field.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Zeroize)]
pub struct Field25519(pub(crate) MontyFormType);

/// Formula from RFC-8032 (modp_sqrt_m1/sqrt8k5 z)
/// 2^((MODULUS - 1) // 4) % MODULUS
const SQRT_M1: Field25519 = Field25519(Field25519Params::SQRT_M1);

/// Constant useful in sqrt_ratio_i (sqrt(u / v))
/// MOD_3_8 - 1
struct FixedExpMod5_8;
impl FixedExponent for FixedExpMod5_8 {
    const EXPONENT: U256 = MontyFormType::new(&U256::from_be_hex(
        "0ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd",
    ))
    .retrieve();
}

impl ConstantTimeEq for Field25519 {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0).into()
    }
}

impl ConditionallySelectable for Field25519 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        // TODO: test if this is ever called
        let c = CtChoice::from(choice);
        Self(MontyFormType::ct_select(&a.0, &b.0, c))
    }
}

impl Add<Self> for Field25519 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self(MontyFormType::add(&self.0, &other.0))
    }
}

impl AddAssign<Self> for Field25519 {
    fn add_assign(&mut self, other: Self) {
        self.0 = MontyFormType::add(&self.0, &other.0);
    }
}

impl<'a> Add<&'a Self> for Field25519 {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        Self(MontyFormType::add(&self.0, &other.0))
    }
}

impl<'a> AddAssign<&'a Self> for Field25519 {
    fn add_assign(&mut self, other: &'a Self) {
        self.0 = MontyFormType::add(&self.0, &other.0);
    }
}

impl Sub<Self> for Field25519 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self(MontyFormType::sub(&self.0, &other.0))
    }
}

impl SubAssign<Self> for Field25519 {
    fn sub_assign(&mut self, other: Self) {
        self.0 = MontyFormType::sub(&self.0, &other.0);
    }
}

impl<'a> Sub<&'a Self> for Field25519 {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        Self(MontyFormType::sub(&self.0, &other.0))
    }
}

impl<'a> SubAssign<&'a Self> for Field25519 {
    fn sub_assign(&mut self, other: &'a Self) {
        self.0 = MontyFormType::sub(&self.0, &other.0);
    }
}

impl Mul<Self> for Field25519 {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        Self(MontyFormType::mul(&self.0, &other.0))
    }
}

impl MulAssign<Self> for Field25519 {
    fn mul_assign(&mut self, other: Self) {
        self.0 = MontyFormType::mul(&self.0, &other.0);
    }
}

impl<'a> Mul<&'a Self> for Field25519 {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        Self(MontyFormType::mul(&self.0, &other.0))
    }
}

impl<'a> MulAssign<&'a Self> for Field25519 {
    fn mul_assign(&mut self, other: &'a Self) {
        self.0 = MontyFormType::mul(&self.0, &other.0);
    }
}

impl From<u8> for Field25519 {
    fn from(a: u8) -> Self {
        Self(MontyFormType::new(&U256::from_u64(u64::from(a))))
    }
}

impl From<u16> for Field25519 {
    fn from(a: u16) -> Self {
        Self(MontyFormType::new(&U256::from_u64(u64::from(a))))
    }
}

impl From<u32> for Field25519 {
    fn from(a: u32) -> Self {
        Self(MontyFormType::new(&U256::from_u64(u64::from(a))))
    }
}

impl From<u64> for Field25519 {
    fn from(a: u64) -> Self {
        Self(MontyFormType::new(&U256::from_u64(a)))
    }
}

impl From<u128> for Field25519 {
    fn from(a: u128) -> Self {
        Self(MontyFormType::new(&U256::from_u128(a)))
    }
}

impl Neg for Field25519 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(MontyForm::neg(&self.0))
    }
}

impl Neg for &Field25519 {
    type Output = Field25519;

    fn neg(self) -> Self::Output {
        Field25519(MontyForm::neg(&self.0))
    }
}

impl Field for Field25519 {
    const ONE: Self = Self(MontyFormType::ONE);
    const ZERO: Self = Self(MontyFormType::ZERO);

    fn random(mut rng: impl RngCore) -> Self {
        Self(MontyFormType::random(&mut rng))
    }

    #[inline]
    fn square(&self) -> Self {
        Self(MontyFormType::square(&self.0))
    }

    #[inline]
    fn double(&self) -> Self {
        Self(MontyFormType::add(&self.0, &self.0))
    }

    fn invert(&self) -> CtOption<Self> {
        let (res, c) = MontyFormType::invert(&self.0);
        CtOption::new(Self(res), c.into())
    }

    #[inline]
    fn sqrt(&self) -> CtOption<Self> {
        let (res, c) = MontyFormType::sqrt(&self.0);
        CtOption::new(Self(res), c.into())
    }

    fn sqrt_ratio(u: &Self, v: &Self) -> (Choice, Self) {
        // NOTE: Used by tests, not by benchmarking.
        let i = SQRT_M1.0;

        let u = &u.0;
        let v = &v.0;

        let v3 = MontyFormType::mul(&v.square(), v);
        let v7 = MontyFormType::mul(&v3.square(), v);
        let u_times_v3 = MontyFormType::mul(u, &v3);
        let u_times_v7 = MontyFormType::mul(u, &v7);
        let mut r = MontyFormType::mul(&u_times_v3, &u_times_v7.pow_fixed::<FixedExpMod5_8>());

        let check = MontyFormType::mul(v, &r.square());
        let correct_sign = check.ct_eq(u);
        let u_neg = MontyFormType::neg(u);
        let flipped_sign = check.ct_eq(&u_neg);
        let flipped_sign_i = check.ct_eq(&MontyFormType::mul(&u_neg, &i));

        r = MontyFormType::ct_select(
            &r,
            &MontyFormType::mul(&r, &i),
            flipped_sign.or(flipped_sign_i),
        );

        let r_is_negative = r.retrieve().ct_is_odd();
        r = MontyFormType::ct_select(&r, &r.neg(), r_is_negative);

        (correct_sign.or(flipped_sign).into(), Self(r))
    }
}

impl PrimeField for Field25519 {
    type Repr = [u8; 32];

    const CAPACITY: u32 = 254;
    // This was calculated via the formula from the ff crate docs
    // Self::MULTIPLICATIVE_GENERATOR ** (2 ** Self::S)
    const DELTA: Self = Self(MontyFormType::new(&U256::from_u64(0x10)));
    // Big endian representation of the modulus
    const MODULUS: &'static str = MODULUS_HEX;
    // This was calculated with the method from the ff crate docs
    // SageMath GF(modulus).primitive_element()
    const MULTIPLICATIVE_GENERATOR: Self = Self(MontyFormType::new(&U256::from_u64(2)));
    const NUM_BITS: u32 = 255;
    // This was calculated via the formula from the ff crate docs
    // Self::MULTIPLICATIVE_GENERATOR ** ((modulus - 1) >> Self::S)
    const ROOT_OF_UNITY: Self = SQRT_M1;
    // Self::ROOT_OF_UNITY.invert()
    const ROOT_OF_UNITY_INV: Self = Self(Self::ROOT_OF_UNITY.0.invert().0);
    // This was set per the specification in the ff crate docs
    // The number of leading zero bits in the little-endian bit representation of (modulus - 1)
    const S: u32 = 2;
    const TWO_INV: Self = Self(MontyFormType::new(&U256::from_u64(2)).invert().0);

    fn from_repr(bytes: [u8; 32]) -> CtOption<Self> {
        let (res, c) = MontyFormType::from_le_bytes(bytes);
        CtOption::new(Self(res), c.into())
    }

    fn to_repr(&self) -> [u8; 32] {
        self.0.to_le_bytes()
    }

    fn is_odd(&self) -> Choice {
        self.0.retrieve().is_odd().into()
    }

    fn from_u128(num: u128) -> Self {
        Self::from(num)
    }
}

impl PrimeFieldBits for Field25519 {
    type ReprBits = [u8; 32];

    fn to_le_bits(&self) -> FieldBits<Self::ReprBits> {
        self.to_repr().into()
    }

    fn char_le_bits() -> FieldBits<Self::ReprBits> {
        Field25519Params::MODULUS.to_le_bytes().into()
    }
}

impl Field25519 {
    pub const fn from_be_hex(hex: &str) -> Self {
        Self(MontyFormType::new(&U256::from_be_hex(hex)))
    }
}

impl Sum<Self> for Field25519 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = MontyForm::ZERO;
        for item in iter {
            res = MontyFormType::add(&res, &item.0);
        }
        Self(res)
    }
}

impl<'a> Sum<&'a Self> for Field25519 {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product<Self> for Field25519 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = MontyForm::ONE;
        for item in iter {
            res = MontyFormType::mul(&res, &item.0);
        }
        Self(res)
    }
}

impl<'a> Product<&'a Self> for Field25519 {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field25519() {
        ff_group_tests::prime_field::test_prime_field_bits::<_, Field25519>(&mut rand_core::OsRng);
    }
}

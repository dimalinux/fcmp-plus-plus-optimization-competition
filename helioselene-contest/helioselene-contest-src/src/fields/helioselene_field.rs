use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ff::{helpers::sqrt_ratio_generic, Field, FieldBits, PrimeField, PrimeFieldBits};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::DefaultIsZeroes;

use crate::u256::{MontyForm, MontyParams, U256};

const MODULUS_STR: &str = "7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79f";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct HelioseleneParams;

impl MontyParams for HelioseleneParams {
    const MODULUS: U256 = U256::from_be_hex(MODULUS_STR);
    /// MODULUS_BITS is the number of bits in the modulus (ours has one leading zero bit)
    const MODULUS_BITS: usize = 255;
    /// MOD_NEG_INV is the modular multiplicative inverse of the least
    /// significant 64-bits of `MODULUS` modulo 2^64, negated.
    const MOD_NEG_INV: u64 = 0x8a5f094bd6f46ba1_u64;
    /// MOD_PLUS_1_DIV_4 is (MODULUS+1) // 4. Used for sqrt.
    const MOD_PLUS_1_DIV_4: U256 =
        U256::from_be_hex("1fffffffffffffffffffffffffffffffefdfde0b2dd95ad61badb49c9e49f1e8");
    /// R is U256::MAX % MODULUS + 1
    const R: U256 =
        U256::from_be_hex("0000000000000000000000000000000081010fa69135294f22925b1b0db070c2");
    /// R2 is R^2 mod MODULUS
    const R2: U256 =
        U256::from_be_hex("410211c6fe99a770f19be179bd15cd0c6e709b56a6427587796519faf06a5304");
    /// R3 is the montgomery form of R2^2
    const R3: U256 =
        U256::from_be_hex("233029c4d15001cac205e02ae548627197f0fb0a46f8b46253018af9d307af37");
    /// TWO_TO_256_MOD_M is 2^256 mod MODULUS
    const TWO_TO_256_MOD_M: U256 =
        U256::from_be_hex("0000000000000000000000000000000081010fa69135294f22925b1b0db070c2");
}

pub(crate) type MontyFormType = MontyForm<HelioseleneParams>;

/// The field novel to Helios/Selene.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct HelioseleneField(pub(crate) MontyForm<HelioseleneParams>);

impl DefaultIsZeroes for HelioseleneField {}

impl ConstantTimeEq for HelioseleneField {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0).into()
    }
}

impl ConditionallySelectable for HelioseleneField {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(MontyForm::ct_select(&a.0, &b.0, choice.into()))
    }
}

impl Add<Self> for HelioseleneField {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0.add(&other.0))
    }
}

impl AddAssign<Self> for HelioseleneField {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(&other.0);
    }
}

impl<'a> Add<&'a Self> for HelioseleneField {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        Self(self.0.add(&other.0))
    }
}

impl<'a> AddAssign<&'a Self> for HelioseleneField {
    fn add_assign(&mut self, other: &'a Self) {
        self.0 = self.0.add(&other.0);
    }
}

impl Sub<Self> for HelioseleneField {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0.sub(&other.0))
    }
}

impl SubAssign<Self> for HelioseleneField {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub(&other.0);
    }
}

impl<'a> Sub<&'a Self> for HelioseleneField {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        Self(self.0.sub(&other.0))
    }
}

impl<'a> SubAssign<&'a Self> for HelioseleneField {
    fn sub_assign(&mut self, other: &'a Self) {
        self.0 = self.0.sub(&other.0);
    }
}

impl Mul<Self> for HelioseleneField {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0.mul(&other.0))
    }
}

impl MulAssign<Self> for HelioseleneField {
    fn mul_assign(&mut self, other: Self) {
        self.0 = self.0.mul(&other.0);
    }
}

impl<'a> Mul<&'a Self> for HelioseleneField {
    type Output = Self;

    #[inline]
    fn mul(self, other: &'a Self) -> Self::Output {
        Self(self.0.mul(&other.0))
    }
}

impl<'a> MulAssign<&'a Self> for HelioseleneField {
    fn mul_assign(&mut self, other: &'a Self) {
        self.0 = self.0.mul(&other.0);
    }
}

impl From<u8> for HelioseleneField {
    fn from(a: u8) -> Self {
        Self(MontyForm::new(&U256::from_u64(u64::from(a))))
    }
}

impl From<u16> for HelioseleneField {
    fn from(a: u16) -> Self {
        Self(MontyForm::new(&U256::from_u64(u64::from(a))))
    }
}

impl From<u32> for HelioseleneField {
    fn from(a: u32) -> Self {
        Self(MontyForm::new(&U256::from_u64(u64::from(a))))
    }
}

impl From<u64> for HelioseleneField {
    fn from(a: u64) -> Self {
        Self(MontyForm::new(&U256::from_u64(a)))
    }
}

impl From<u128> for HelioseleneField {
    fn from(a: u128) -> Self {
        Self(MontyForm::new(&U256::from_u128(a)))
    }
}

impl Neg for HelioseleneField {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Neg for &HelioseleneField {
    type Output = HelioseleneField;

    fn neg(self) -> Self::Output {
        (*self).neg()
    }
}

impl HelioseleneField {
    /// Perform exponentiation.
    // NOTE: Invoked by cycles tests, but not benchmarked on Intel.
    #[must_use]
    #[inline]
    pub const fn pow(&self, exponent: Self) -> Self {
        Self(self.0.pow(exponent.0))
    }

    /// Reduce 512 bits, presumably to get a non-biased Helioselene field element.
    pub fn reduce(bytes: &[u8; 64]) -> Self {
        Self(MontyFormType::reduce(bytes))
    }

    pub const fn from_be_hex(hex: &str) -> Self {
        Self(MontyFormType::new(&U256::from_be_hex(hex)))
    }
}

impl Field for HelioseleneField {
    const ONE: Self = Self(MontyForm::ONE);
    const ZERO: Self = Self(MontyForm::ZERO);

    fn random(mut rng: impl RngCore) -> Self {
        Self(MontyFormType::random(&mut rng))
    }

    #[inline]
    fn square(&self) -> Self {
        Self(self.0.square())
    }

    #[inline]
    fn double(&self) -> Self {
        Self(self.0.double())
    }

    fn invert(&self) -> CtOption<Self> {
        let (res, c) = self.0.invert();
        CtOption::new(Self(res), c.into())
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        sqrt_ratio_generic(num, div)
    }

    #[inline]
    fn sqrt(&self) -> CtOption<Self> {
        let (res, c) = self.0.sqrt();
        CtOption::new(Self(res), c.into())
    }
}

impl PrimeField for HelioseleneField {
    type Repr = [u8; 32];

    const CAPACITY: u32 = 254;
    const DELTA: Self =
        Self::from_be_hex("0000000000000000000000000000000000000000000000000000000000000019");
    const MODULUS: &'static str = MODULUS_STR;
    const MULTIPLICATIVE_GENERATOR: Self = Self(MontyForm::new(&U256::from_u64(5)));
    const NUM_BITS: u32 = 255;
    const ROOT_OF_UNITY: Self =
        Self::from_be_hex("7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79e");
    const ROOT_OF_UNITY_INV: Self = Self(Self::ROOT_OF_UNITY.0.invert().0);
    const S: u32 = 1;
    const TWO_INV: Self = Self(MontyFormType::new(&U256::from_u64(2)).invert().0);

    fn from_repr(bytes: Self::Repr) -> CtOption<Self> {
        let (res, c) = MontyFormType::from_le_bytes(bytes);
        CtOption::new(Self(res), c.into())
    }

    fn to_repr(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }

    fn is_odd(&self) -> Choice {
        self.0.retrieve().is_odd().into()
    }
}

impl PrimeFieldBits for HelioseleneField {
    type ReprBits = [u8; 32];

    fn to_le_bits(&self) -> FieldBits<Self::ReprBits> {
        self.to_repr().into()
    }

    fn char_le_bits() -> FieldBits<Self::ReprBits> {
        let mut repr = [0; 32];
        repr.copy_from_slice(&HelioseleneParams::MODULUS.to_le_bytes());
        repr.into()
    }
}

impl Sum<Self> for HelioseleneField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = MontyFormType::ZERO;
        for item in iter {
            res = res.add(&item.0);
        }
        Self(res)
    }
}

impl<'a> Sum<&'a Self> for HelioseleneField {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product<Self> for HelioseleneField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = MontyFormType::ONE;
        for item in iter {
            res = res.mul(&item.0);
        }
        Self(res)
    }
}

impl<'a> Product<&'a Self> for HelioseleneField {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helioselene_field() {
        ff_group_tests::prime_field::test_prime_field_bits::<_, HelioseleneField>(
            &mut rand_core::OsRng,
        );
    }
}

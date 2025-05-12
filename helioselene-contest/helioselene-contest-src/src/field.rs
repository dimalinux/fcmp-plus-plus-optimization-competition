use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ff::{helpers::sqrt_ratio_generic, Field, FieldBits, PrimeField, PrimeFieldBits};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::{DefaultIsZeroes, Zeroize};

use crate::{
    backend::u8_from_bool,
    u256::{Encoding, Residue, ResidueParams, U256},
};

const MODULUS_STR: &str = "7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79f";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct HelioseleneQ;

impl ResidueParams for HelioseleneQ {
    const MODULUS: U256 = U256::from_be_hex(MODULUS_STR);
    /// MOD_NEG_INV is the modular multiplicative inverse of the least
    /// significant 64-bits of `MODULUS` modulo 2^64, negated.
    const MOD_NEG_INV: u64 = 0x8a5f094bd6f46ba1_u64;
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

pub(crate) type ResidueType = Residue<HelioseleneQ>;

/// The field novel to Helios/Selene.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct HelioseleneField(pub(crate) ResidueType);

impl DefaultIsZeroes for HelioseleneField {}

impl ConstantTimeEq for HelioseleneField {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl ConditionallySelectable for HelioseleneField {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(Residue::conditional_select(&a.0, &b.0, choice))
    }
}
impl Add<Self> for HelioseleneField {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(ResidueType::add(&self.0, &other.0))
    }
}
impl AddAssign<Self> for HelioseleneField {
    fn add_assign(&mut self, other: Self) {
        self.0 = ResidueType::add(&self.0, &other.0);
    }
}
impl<'a> Add<&'a Self> for HelioseleneField {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        Self(ResidueType::add(&self.0, &other.0))
    }
}
impl<'a> AddAssign<&'a Self> for HelioseleneField {
    fn add_assign(&mut self, other: &'a Self) {
        self.0 = ResidueType::add(&self.0, &other.0);
    }
}
impl Sub<Self> for HelioseleneField {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(ResidueType::sub(&self.0, &other.0))
    }
}
impl SubAssign<Self> for HelioseleneField {
    fn sub_assign(&mut self, other: Self) {
        self.0 = ResidueType::sub(&self.0, &other.0);
    }
}
impl<'a> Sub<&'a Self> for HelioseleneField {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        Self(ResidueType::sub(&self.0, &other.0))
    }
}
impl<'a> SubAssign<&'a Self> for HelioseleneField {
    fn sub_assign(&mut self, other: &'a Self) {
        self.0 = ResidueType::sub(&self.0, &other.0);
    }
}
impl Mul<Self> for HelioseleneField {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(ResidueType::mul(&self.0, &other.0))
    }
}
impl MulAssign<Self> for HelioseleneField {
    fn mul_assign(&mut self, other: Self) {
        self.0 = ResidueType::mul(&self.0, &other.0);
    }
}
impl<'a> Mul<&'a Self> for HelioseleneField {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self::Output {
        Self(ResidueType::mul(&self.0, &other.0))
    }
}
impl<'a> MulAssign<&'a Self> for HelioseleneField {
    fn mul_assign(&mut self, other: &'a Self) {
        self.0 = ResidueType::mul(&self.0, &other.0);
    }
}
impl From<u8> for HelioseleneField {
    fn from(a: u8) -> Self {
        Self(Residue::new(&U256::from_u64(u64::from(a))))
    }
}
impl From<u16> for HelioseleneField {
    fn from(a: u16) -> Self {
        Self(Residue::new(&U256::from_u64(u64::from(a))))
    }
}
impl From<u32> for HelioseleneField {
    fn from(a: u32) -> Self {
        Self(Residue::new(&U256::from_u64(u64::from(a))))
    }
}
impl From<u64> for HelioseleneField {
    fn from(a: u64) -> Self {
        Self(Residue::new(&U256::from_u64(a)))
    }
}
impl From<u128> for HelioseleneField {
    fn from(a: u128) -> Self {
        Self(Residue::new(&U256::from_u128(a)))
    }
}
impl Neg for HelioseleneField {
    type Output = Self;

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
    #[must_use]
    pub fn pow(&self, other: Self) -> Self {
        let mut table = [Self(Residue::ONE); 16];
        table[1] = *self;
        for i in 2..16 {
            table[i] = table[i - 1] * self;
        }
        let mut res = Self(Residue::ONE);
        let mut bits = 0;
        for (i, mut bit) in other.to_le_bits().iter_mut().rev().enumerate() {
            bits <<= 1;
            let mut bit = u8_from_bool(&mut bit);
            bits |= bit;
            bit.zeroize();
            if ((i + 1) % 4) == 0 {
                if i != 3 {
                    for _ in 0..4 {
                        res *= res;
                    }
                }
                let mut factor = table[0];
                for (j, candidate) in table[1..].iter().enumerate() {
                    let j = j + 1;
                    factor =
                        Self::conditional_select(&factor, candidate, usize::from(bits).ct_eq(&j));
                }
                res *= factor;
                bits = 0;
            }
        }
        res
    }

    /// Reduce 512 bits, presumably to get a non-biased Helioselene field element.
    /// While taking the modulus of 512 bits produces negligible bias (method used
    /// below), there may be better algorithms with zero bias.
    pub(crate) fn reduce(bytes: &[u8; 64]) -> Self {
        // Do modulus on 512 bits using 256-bit math
        // val_512 mod M = (((2^256 mod M) * hi_256) mod M + (lo_256 mod M)) mod M
        const TWO_TO_256_MOD_M: ResidueType = ResidueType::new(&HelioseleneQ::TWO_TO_256_MOD_M);
        let lo = ResidueType::new(&U256::from_le_slice(&bytes[..32]));
        let hi = ResidueType::new(&U256::from_le_slice(&bytes[32..64]));
        let hi = ResidueType::mul(&TWO_TO_256_MOD_M, &hi);
        Self(ResidueType::add(&hi, &lo))
    }
}
impl Field for HelioseleneField {
    const ONE: Self = Self(Residue::ONE);
    const ZERO: Self = Self(Residue::ZERO);

    fn random(mut rng: impl RngCore) -> Self {
        let mut bytes = [0; 64];
        rng.fill_bytes(&mut bytes);
        Self::reduce(&bytes)
    }

    fn square(&self) -> Self {
        Self(ResidueType::square(&self.0))
    }

    fn double(&self) -> Self {
        Self(ResidueType::add(&self.0, &self.0))
    }

    fn invert(&self) -> CtOption<Self> {
        let res = self.0.invert();
        CtOption::new(Self(res.0), res.1.into())
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        sqrt_ratio_generic(num, div)
    }

    fn sqrt(&self) -> CtOption<Self> {
        const MOD_PLUS_ONE_DIV_FOUR: U256 = HelioseleneQ::MODULUS
            .saturating_add(&U256::ONE)
            .wrapping_div(&U256::from_u64(4));
        // TODO: below is using checked on a constant
        let res = self.pow(Self(
            ResidueType::new_checked(&MOD_PLUS_ONE_DIV_FOUR).unwrap(),
        ));
        CtOption::new(res, res.square().ct_eq(self))
    }
}
impl PrimeField for HelioseleneField {
    type Repr = [u8; 32];

    const CAPACITY: u32 = 254;
    const DELTA: Self = Self(Residue::new(&U256::from_be_hex(
        "0000000000000000000000000000000000000000000000000000000000000019",
    )));
    const MODULUS: &'static str = MODULUS_STR;
    const MULTIPLICATIVE_GENERATOR: Self = Self(Residue::new(&U256::from_u64(5)));
    const NUM_BITS: u32 = 255;
    const ROOT_OF_UNITY: Self = Self(Residue::new(&U256::from_be_hex(
        "7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79e",
    )));
    const ROOT_OF_UNITY_INV: Self = Self(Self::ROOT_OF_UNITY.0.invert().0);
    const S: u32 = 1;
    const TWO_INV: Self = Self(ResidueType::new(&U256::from_u64(2)).invert().0);

    fn from_repr(bytes: Self::Repr) -> CtOption<Self> {
        let res = U256::from_le_slice(&bytes);
        CtOption::new(
            Self(Residue::new(&res)),
            U256::ct_lt(&res, &HelioseleneQ::MODULUS).into(),
        )
    }

    fn to_repr(&self) -> Self::Repr {
        let mut repr = [0; 32];
        repr.copy_from_slice(&self.0.retrieve().to_le_bytes());
        repr
    }

    fn is_odd(&self) -> Choice {
        self.0.retrieve().is_odd()
    }
}
impl PrimeFieldBits for HelioseleneField {
    type ReprBits = [u8; 32];

    fn to_le_bits(&self) -> FieldBits<Self::ReprBits> {
        self.to_repr().into()
    }

    fn char_le_bits() -> FieldBits<Self::ReprBits> {
        let mut repr = [0; 32];
        repr.copy_from_slice(&HelioseleneQ::MODULUS.to_le_bytes());
        repr.into()
    }
}
impl Sum<Self> for HelioseleneField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = ResidueType::ZERO;
        for item in iter {
            res = ResidueType::add(&res, &item.0);
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
        let mut res = ResidueType::ONE;
        for item in iter {
            res = ResidueType::mul(&res, &item.0);
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

    #[test]
    fn test_reduce_helioselene_field() {
        struct TC {
            input: &'static str,
            output: &'static str,
        }

        // Hex values are in big endian.
        const REDUCE_TESTS: [TC; 4] = [
            TC {
                input: "70b7f6776fedc692aaa93223b6694532d97205e209f2e2cb51b49c056988041780d802b0513e6a11e7ece450e3166ce4d8a13a56cdeb3c5d731c4cac2d9bc9a1",
                output: "4c854e33959c8db9bf70bd7e1570b9b4c79b0cfa4371f9021422286907c70a3c",

            },
            TC {
                input: "0a6dc2d2be742c5d0d811ee43afeef432c8d529332ad7ca541d1477b5276ede8ade6b16414b5a165ef8d94f908036056f88d5228d9f9479e247e632c9de9715f",
                output: "49e818746c572bb613708a36e45a3b70e7167c1d01773d8d8e72149e0f3e16d8",

            },
            TC {
                input: "f23e13f70f8369004e2b0e06772676b4f827111bc2961f80c738aca2ac6a92c638c5a561f78952fd1dff02e2078e0ea7c49e4a1a6939cf3b8304c8ee9e31f4ef",
                output: "7e74749f65cd5ddb47d587453a0e98ecb9dcee837cbbdfd78c83ba1fd8981529",

            },
            TC {
                input: "d32b624c8176b0d0ed780fbdc248f7df4e862e110a9bc03624ba0ebff2d9f55906d9769ab1bcde613af3e3417805b728dd025f6c0cfc87209faeb2f484cacc08",
                output: "24c6031703a130382415c0cd23f7a04c597ee6fcacb2df0c00e65fbf4dd7aff6",

            }
        ];

        for tc in &REDUCE_TESTS {
            let mut input: [u8; 64] = hex::decode(tc.input)
                .expect("Failed to decode hex")
                .try_into()
                .expect("Input must be 64 bytes");
            input.reverse(); // Use little endian once in binary form

            let output = HelioseleneField::reduce(&input);
            let ouput = hex::encode(output.0.retrieve().to_be_bytes());
            assert_eq!(ouput, tc.output);
        }
    }
}

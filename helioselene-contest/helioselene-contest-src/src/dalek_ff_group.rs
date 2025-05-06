use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use group::ff::{Field, FieldBits, PrimeField, PrimeFieldBits};
use rand_core::RngCore;
use subtle::{
    Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess,
    CtOption,
};
use zeroize::Zeroize;

use crate::{
    backend::u8_from_bool,
    bigint::{montgomery_reduction, Encoding, Limb, Residue, ResidueParams, Word, U256},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub(crate) struct FieldModulus {}
impl ResidueParams for FieldModulus {
    // MODULUS is: 2^255 - 19
    const MODULUS: U256 = {
        let res =
            U256::from_be_hex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed");

        if res.as_limbs()[0].0 & 1 == 0 {
            panic!("modulus must be odd");
        }

        res
    };
    const MOD_NEG_INV: Limb = Limb(
        Word::MIN.wrapping_sub(
            Self::MODULUS
                .inv_mod2k_vartime(Word::BITS as usize)
                .as_limbs()[0]
                .0,
        ),
    );
    const R: U256 = U256::MAX
        .const_rem(&Self::MODULUS)
        .0
        .wrapping_add(&U256::ONE);
    const R2: U256 = U256::const_rem_wide(Self::R.square_wide(), &Self::MODULUS).0;
    const R3: U256 =
        montgomery_reduction(&Self::R2.square_wide(), &Self::MODULUS, Self::MOD_NEG_INV);
    const TWO_TO_256_MOD_M: U256 =
        U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000026");
}
pub(crate) type ResidueType = Residue<FieldModulus>;

/// A constant-time implementation of the Ed25519 field.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Zeroize)]
#[repr(C)]
pub struct Field25519(pub(crate) ResidueType);

// Square root of -1.
// Formula from RFC-8032 (modp_sqrt_m1/sqrt8k5 z)
// 2 ** ((MODULUS - 1) // 4) % MODULUS
const SQRT_M1: Field25519 = Field25519(
    ResidueType::new(&U256::from_u8(2)).pow(
        &FieldModulus::MODULUS
            .saturating_sub(&U256::ONE)
            .wrapping_div(&U256::from_u8(4)),
    ),
);

// Constant useful in calculating square roots (RFC-8032 sqrt8k5's exponent used to calculate y)
const MOD_3_8: Field25519 = Field25519(ResidueType::new(
    &FieldModulus::MODULUS
        .saturating_add(&U256::from_u8(3))
        .wrapping_div(&U256::from_u8(8)),
));

// Constant useful in sqrt_ratio_i (sqrt(u / v))
const MOD_5_8: Field25519 = Field25519(ResidueType::sub(&MOD_3_8.0, &ResidueType::ONE));

impl ConstantTimeEq for Field25519 {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl ConditionallySelectable for Field25519 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Field25519(ResidueType::conditional_select(&a.0, &b.0, choice))
    }
}

impl Add<Field25519> for Field25519 {
    type Output = Field25519;

    fn add(self, other: Field25519) -> Self::Output {
        Self(ResidueType::add(&self.0, &other.0))
    }
}
impl AddAssign<Field25519> for Field25519 {
    fn add_assign(&mut self, other: Field25519) {
        self.0 = ResidueType::add(&self.0, &other.0);
    }
}
impl<'a> Add<&'a Field25519> for Field25519 {
    type Output = Field25519;

    fn add(self, other: &'a Field25519) -> Self::Output {
        Self(ResidueType::add(&self.0, &other.0))
    }
}
impl<'a> AddAssign<&'a Field25519> for Field25519 {
    fn add_assign(&mut self, other: &'a Field25519) {
        self.0 = ResidueType::add(&self.0, &other.0);
    }
}
impl Sub<Field25519> for Field25519 {
    type Output = Field25519;

    fn sub(self, other: Field25519) -> Self::Output {
        Self(ResidueType::sub(&self.0, &other.0))
    }
}
impl SubAssign<Field25519> for Field25519 {
    fn sub_assign(&mut self, other: Field25519) {
        self.0 = ResidueType::sub(&self.0, &other.0);
    }
}
impl<'a> Sub<&'a Field25519> for Field25519 {
    type Output = Field25519;

    fn sub(self, other: &'a Field25519) -> Self::Output {
        Self(ResidueType::sub(&self.0, &other.0))
    }
}
impl<'a> SubAssign<&'a Field25519> for Field25519 {
    fn sub_assign(&mut self, other: &'a Field25519) {
        self.0 = ResidueType::sub(&self.0, &other.0);
    }
}
impl Mul<Field25519> for Field25519 {
    type Output = Field25519;

    fn mul(self, other: Field25519) -> Self::Output {
        Self(ResidueType::mul(&self.0, &other.0))
    }
}
impl MulAssign<Field25519> for Field25519 {
    fn mul_assign(&mut self, other: Field25519) {
        self.0 = ResidueType::mul(&self.0, &other.0);
    }
}
impl<'a> Mul<&'a Field25519> for Field25519 {
    type Output = Field25519;

    fn mul(self, other: &'a Field25519) -> Self::Output {
        Self(ResidueType::mul(&self.0, &other.0))
    }
}
impl<'a> MulAssign<&'a Field25519> for Field25519 {
    fn mul_assign(&mut self, other: &'a Field25519) {
        self.0 = ResidueType::mul(&self.0, &other.0);
    }
}

impl From<u8> for Field25519 {
    fn from(a: u8) -> Field25519 {
        Self(ResidueType::new(&U256::from(a)))
    }
}

impl From<u16> for Field25519 {
    fn from(a: u16) -> Field25519 {
        Self(ResidueType::new(&U256::from_u16(a)))
    }
}

impl From<u32> for Field25519 {
    fn from(a: u32) -> Field25519 {
        Self(ResidueType::new(&U256::from_u32(a)))
    }
}

impl From<u64> for Field25519 {
    fn from(a: u64) -> Field25519 {
        Self(ResidueType::new(&U256::from_u64(a)))
    }
}

impl From<u128> for Field25519 {
    fn from(a: u128) -> Field25519 {
        Self(ResidueType::new(&U256::from_u128(a)))
    }
}

impl Neg for Field25519 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(Residue::neg(&self.0))
    }
}

impl Neg for &Field25519 {
    type Output = Field25519;

    fn neg(self) -> Self::Output {
        Field25519(Residue::neg(&self.0))
    }
}

impl Field for Field25519 {
    const ONE: Self = Self(ResidueType::ONE);
    const ZERO: Self = Self(ResidueType::ZERO);

    fn random(mut rng: impl RngCore) -> Self {
        let mut bytes = [0; 64];
        rng.fill_bytes(&mut bytes);
        Self::reduce(&bytes)
    }

    fn square(&self) -> Self {
        Field25519(ResidueType::square(&self.0))
    }

    fn double(&self) -> Self {
        Field25519(ResidueType::add(&self.0, &self.0))
    }

    fn invert(&self) -> CtOption<Self> {
        let res = ResidueType::invert(&self.0);
        CtOption::new(Self(res.0), res.1.into())
    }

    // RFC-8032 sqrt8k5
    fn sqrt(&self) -> CtOption<Self> {
        let tv1 = self.pow(MOD_3_8);
        let tv2 = tv1 * SQRT_M1;
        let candidate = Self::conditional_select(&tv2, &tv1, tv1.square().ct_eq(self));
        CtOption::new(candidate, candidate.square().ct_eq(self))
    }

    fn sqrt_ratio(u: &Field25519, v: &Field25519) -> (Choice, Field25519) {
        let i = SQRT_M1;

        let u = *u;
        let v = *v;

        let v3 = v.square() * v;
        let v7 = v3.square() * v;
        let mut r = (u * v3) * (u * v7).pow(MOD_5_8);

        let check = v * r.square();
        let correct_sign = check.ct_eq(&u);
        let flipped_sign = check.ct_eq(&(-u));
        let flipped_sign_i = check.ct_eq(&((-u) * i));

        r.conditional_assign(&(r * i), flipped_sign | flipped_sign_i);

        let r_is_negative = r.is_odd();
        r.conditional_negate(r_is_negative);

        (correct_sign | flipped_sign, r)
    }
}

impl PrimeField for Field25519 {
    type Repr = [u8; 32];

    const CAPACITY: u32 = 254;
    // This was calculated via the formula from the ff crate docs
    // Self::MULTIPLICATIVE_GENERATOR ** (2 ** Self::S)
    const DELTA: Self = Field25519(ResidueType::new(&U256::from_be_hex(
        "0000000000000000000000000000000000000000000000000000000000000010",
    )));
    // Big endian representation of the modulus
    const MODULUS: &'static str =
        "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed";
    // This was calculated with the method from the ff crate docs
    // SageMath GF(modulus).primitive_element()
    const MULTIPLICATIVE_GENERATOR: Self = Self(ResidueType::new(&U256::from_u8(2)));
    const NUM_BITS: u32 = 255;
    // This was calculated via the formula from the ff crate docs
    // Self::MULTIPLICATIVE_GENERATOR ** ((modulus - 1) >> Self::S)
    const ROOT_OF_UNITY: Self = Field25519(ResidueType::new(&U256::from_be_hex(
        "2b8324804fc1df0b2b4d00993dfbd7a72f431806ad2fe478c4ee1b274a0ea0b0",
    )));
    // Self::ROOT_OF_UNITY.invert()
    const ROOT_OF_UNITY_INV: Self = Field25519(Self::ROOT_OF_UNITY.0.invert().0);
    // This was set per the specification in the ff crate docs
    // The number of leading zero bits in the little-endian bit representation of (modulus - 1)
    const S: u32 = 2;
    const TWO_INV: Self = Field25519(ResidueType::new(&U256::from_u8(2)).invert().0);

    fn from_repr(bytes: [u8; 32]) -> CtOption<Self> {
        let res = U256::from_le_bytes(bytes);
        CtOption::new(
            Self(ResidueType::new(&res)),
            res.ct_lt(&FieldModulus::MODULUS),
        )
    }

    fn to_repr(&self) -> [u8; 32] {
        self.0.retrieve().to_le_bytes()
    }

    fn is_odd(&self) -> Choice {
        self.0.retrieve().is_odd()
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
        FieldModulus::MODULUS.to_le_bytes().into()
    }
}

impl Field25519 {
    /// Perform an exponentiation.
    pub fn pow(&self, other: Field25519) -> Field25519 {
        let mut table = [Field25519::ONE; 16];
        table[1] = *self;
        for i in 2..16 {
            table[i] = table[i - 1] * self;
        }

        let mut res = Field25519::ONE;
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
        const TWO_TO_256_MOD_M: ResidueType = ResidueType::new(&FieldModulus::TWO_TO_256_MOD_M);
        let lo = ResidueType::new(&U256::from_le_slice(&bytes[..32]));
        let hi = ResidueType::new(&U256::from_le_slice(&bytes[32..64]));
        let hi = ResidueType::mul(&TWO_TO_256_MOD_M, &hi);
        Self(ResidueType::add(&hi, &lo))
    }
}

impl Sum<Field25519> for Field25519 {
    fn sum<I: Iterator<Item = Field25519>>(iter: I) -> Field25519 {
        let mut res = ResidueType::ZERO;
        for item in iter {
            res = ResidueType::add(&res, &item.0);
        }
        Self(res)
    }
}

impl<'a> Sum<&'a Field25519> for Field25519 {
    fn sum<I: Iterator<Item = &'a Field25519>>(iter: I) -> Field25519 {
        iter.copied().sum()
    }
}

impl Product<Field25519> for Field25519 {
    fn product<I: Iterator<Item = Field25519>>(iter: I) -> Field25519 {
        let mut res = ResidueType::ONE;
        for item in iter {
            res = ResidueType::mul(&res, &item.0);
        }
        Self(res)
    }
}

impl<'a> Product<&'a Field25519> for Field25519 {
    fn product<I: Iterator<Item = &'a Field25519>>(iter: I) -> Field25519 {
        iter.copied().product()
    }
}

#[test]
fn test_sqrt_m1() {
    // Test equivalence against the known constant value
    const SQRT_M1_MAGIC: U256 =
        U256::from_be_hex("2b8324804fc1df0b2b4d00993dfbd7a72f431806ad2fe478c4ee1b274a0ea0b0");
    assert_eq!(SQRT_M1.0.retrieve(), SQRT_M1_MAGIC);

    // Also test equivalence against the result of the formula from RFC-8032 (modp_sqrt_m1/sqrt8k5 z)
    // 2 ** ((MODULUS - 1) // 4) % MODULUS
    assert_eq!(
        SQRT_M1,
        Field25519::from(2u8).pow(Field25519(ResidueType::new(
            &(Field25519::ZERO - Field25519::ONE)
                .0
                .retrieve()
                .wrapping_div(&U256::from(4u8))
        )))
    );
}

#[cfg(test)]
mod tests {
    use crate::{bigint::Encoding, Field25519};

    #[test]
    fn test_field() {
        ff_group_tests::prime_field::test_prime_field_bits::<_, Field25519>(&mut rand_core::OsRng);
    }

    #[test]
    fn test_reduce_field25519() {
        struct TC {
            input: &'static str,
            output: &'static str,
        }

        // Hex values are in big endian.
        const REDUCE_TESTS: [TC; 4] = [
            TC {
                input: "70b7f6776fedc692aaa93223b6694532d97205e209f2e2cb51b49c056988041780d802b0513e6a11e7ece450e3166ce4d8a13a56cdeb3c5d731c4cac2d9bc9a1",
                output: "3c26986aee89e3d73d0a559df6b6b2711f8e19e447f8e68b93eb7579d7cc6791",
            },
            TC {
                input: "0a6dc2d2be742c5d0d811ee43afeef432c8d529332ad7ca541d1477b5276ede8ade6b16414b5a165ef8d94f908036056f88d5228d9f9479e247e632c9de9715f",
                output: "3a319cac59f43735f0b82ad9c9dae44f958794025fb9c825e98eff7adb90c21b",
            },
            TC {
                input: "f23e13f70f8369004e2b0e06772676b4f827111bc2961f80c738aca2ac6a92c638c5a561f78952fd1dff02e2078e0ea7c49e4a1a6939cf3b8304c8ee9e31f4ef",
                output: "2dfc9c0e450ae908b86317d7b743ad849a6ad4394b827c59156e69143603c3ab",
            },
            TC {
                input: "d32b624c8176b0d0ed780fbdc248f7df4e862e110a9bc03624ba0ebff2d9f55906d9769ab1bcde613af3e3417805b728dd025f6c0cfc87209faeb2f484cacc08",
                output: "5f4a0df5e95b1d647ac6396c4eda824e84ed35f3a01b0f2a134ce37291253bd8",
            }
        ];

        for tc in &REDUCE_TESTS {
            let mut input: [u8; 64] = hex::decode(tc.input)
                .expect("Failed to decode hex")
                .try_into()
                .expect("Input must be 64 bytes");
            input.reverse();

            let output = Field25519::reduce(&input);
            let output = hex::encode(output.0.retrieve().to_be_bytes());
            assert_eq!(output, tc.output);
        }
    }
}

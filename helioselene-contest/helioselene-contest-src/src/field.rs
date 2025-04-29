use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use ff::{helpers::sqrt_ratio_generic, Field, FieldBits, PrimeField, PrimeFieldBits};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, ConstantTimeLess, CtOption};
use zeroize::{DefaultIsZeroes, Zeroize};

use crate::{
    backend::u8_from_bool,
    bigint::{
        montgomery_reduction, ConcatMixed, Encoding, Integer, Limb, NonZero, Residue,
        ResidueParams, Uint, Word, U256, U512,
    },
};

const MODULUS_STR: &str = "7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79f";

//impl_modulus!(HelioseleneQ, U256, MODULUS_STR);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HelioseleneQ {}

impl<const DLIMBS: usize> ResidueParams for HelioseleneQ
where
    U256: ConcatMixed<MixedOutput = Uint<DLIMBS>>,
{
    const MODULUS: U256 = {
        let res = <U256>::from_be_hex(MODULUS_STR);

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
    const R: U256 = Uint::MAX
        .const_rem(&Self::MODULUS)
        .0
        .wrapping_add(&Uint::ONE);
    const R2: U256 = Uint::const_rem_wide(Self::R.square_wide(), &Self::MODULUS).0;
    const R3: U256 =
        montgomery_reduction(&Self::R2.square_wide(), &Self::MODULUS, Self::MOD_NEG_INV);
}

type ResidueType = Residue<HelioseleneQ>;

/// The field novel to Helios/Selene.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(C)]
pub struct HelioseleneField(pub(crate) ResidueType);

impl DefaultIsZeroes for HelioseleneField {}

const MODULUS: U256 = U256::from_be_hex(MODULUS_STR);

const WIDE_MODULUS: U512 = U512::from_be_hex(concat!(
    "0000000000000000000000000000000000000000000000000000000000000000",
    "7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79f",
));

fn reduce(x: U512) -> U256 {
    U256::from_le_slice(&x.rem(&NonZero::new(WIDE_MODULUS).unwrap()).to_le_bytes()[..32])
}
impl ConstantTimeEq for HelioseleneField {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}
impl ConditionallySelectable for HelioseleneField {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        HelioseleneField(Residue::conditional_select(&a.0, &b.0, choice))
    }
}
impl Add<HelioseleneField> for HelioseleneField {
    type Output = HelioseleneField;

    fn add(self, other: HelioseleneField) -> Self::Output {
        Self(self.0.add(other.0))
    }
}
impl AddAssign<HelioseleneField> for HelioseleneField {
    fn add_assign(&mut self, other: HelioseleneField) {
        self.0 = self.0.add(&other.0);
    }
}
impl<'a> Add<&'a HelioseleneField> for HelioseleneField {
    type Output = HelioseleneField;

    fn add(self, other: &'a HelioseleneField) -> Self::Output {
        Self(self.0.add(&other.0))
    }
}
impl<'a> AddAssign<&'a HelioseleneField> for HelioseleneField {
    fn add_assign(&mut self, other: &'a HelioseleneField) {
        self.0 = self.0.add(&other.0);
    }
}
impl Sub<HelioseleneField> for HelioseleneField {
    type Output = HelioseleneField;

    fn sub(self, other: HelioseleneField) -> Self::Output {
        Self(self.0.sub(&other.0))
    }
}
impl SubAssign<HelioseleneField> for HelioseleneField {
    fn sub_assign(&mut self, other: HelioseleneField) {
        self.0 = self.0.sub(&other.0);
    }
}
impl<'a> Sub<&'a HelioseleneField> for HelioseleneField {
    type Output = HelioseleneField;

    fn sub(self, other: &'a HelioseleneField) -> Self::Output {
        Self(self.0.sub(&other.0))
    }
}
impl<'a> SubAssign<&'a HelioseleneField> for HelioseleneField {
    fn sub_assign(&mut self, other: &'a HelioseleneField) {
        self.0 = self.0.sub(&other.0);
    }
}
impl Mul<HelioseleneField> for HelioseleneField {
    type Output = HelioseleneField;

    fn mul(self, other: HelioseleneField) -> Self::Output {
        Self(self.0.mul(&other.0))
    }
}
impl MulAssign<HelioseleneField> for HelioseleneField {
    fn mul_assign(&mut self, other: HelioseleneField) {
        self.0 = self.0.mul(&other.0);
    }
}
impl<'a> Mul<&'a HelioseleneField> for HelioseleneField {
    type Output = HelioseleneField;

    fn mul(self, other: &'a HelioseleneField) -> Self::Output {
        Self(self.0.mul(&other.0))
    }
}
impl<'a> MulAssign<&'a HelioseleneField> for HelioseleneField {
    fn mul_assign(&mut self, other: &'a HelioseleneField) {
        self.0 = self.0.mul(&other.0);
    }
}
impl From<u8> for HelioseleneField {
    fn from(a: u8) -> HelioseleneField {
        Self(Residue::new(&U256::from(a)))
    }
}
impl From<u16> for HelioseleneField {
    fn from(a: u16) -> HelioseleneField {
        Self(Residue::new(&U256::from(a)))
    }
}
impl From<u32> for HelioseleneField {
    fn from(a: u32) -> HelioseleneField {
        Self(Residue::new(&U256::from_u32(a)))
    }
}
impl From<u64> for HelioseleneField {
    fn from(a: u64) -> HelioseleneField {
        Self(Residue::new(&U256::from_u64(a)))
    }
}
impl From<u128> for HelioseleneField {
    fn from(a: u128) -> HelioseleneField {
        Self(Residue::new(&U256::from_u128(a)))
    }
}
impl Neg for HelioseleneField {
    type Output = HelioseleneField;

    fn neg(self) -> HelioseleneField {
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
    pub fn pow(&self, other: HelioseleneField) -> HelioseleneField {
        let mut table = [Self(Residue::ONE); 16];
        table[1] = *self;
        for i in 2..16 {
            table[i] = table[i - 1] * self;
        }
        let mut res = Self(Residue::ONE);
        let mut bits = 0;
        for (i, mut bit) in other.to_le_bits().iter_mut().rev().enumerate() {
            bits <<= 1;
            let mut bit = u8_from_bool(bit.deref_mut());
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
}
impl Field for HelioseleneField {
    const ONE: Self = Self(Residue::ONE);
    const ZERO: Self = Self(Residue::ZERO);

    fn random(mut rng: impl RngCore) -> Self {
        let mut bytes = [0; 64];
        rng.fill_bytes(&mut bytes);
        HelioseleneField(Residue::new(&reduce(U512::from_le_slice(bytes.as_ref()))))
    }

    fn square(&self) -> Self {
        Self(self.0.square())
    }

    fn double(&self) -> Self {
        *self + self
    }

    fn invert(&self) -> CtOption<Self> {
        let res = self.0.invert();
        CtOption::new(Self(res.0), res.1.into())
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        sqrt_ratio_generic(num, div)
    }

    fn sqrt(&self) -> CtOption<Self> {
        let mod_plus_one_div_four = MODULUS
            .saturating_add(&U256::ONE)
            .wrapping_div(&(4u8.into()));
        let res = self.pow(Self(
            ResidueType::new_checked(&mod_plus_one_div_four).unwrap(),
        ));
        CtOption::new(res, res.square().ct_eq(self))
    }
}
impl PrimeField for HelioseleneField {
    type Repr = [u8; 32];

    const CAPACITY: u32 = 254;
    const DELTA: Self = HelioseleneField(Residue::new(&U256::from_be_hex(
        "0000000000000000000000000000000000000000000000000000000000000019",
    )));
    const MODULUS: &'static str = MODULUS_STR;
    const MULTIPLICATIVE_GENERATOR: Self = Self(Residue::new(&U256::from_u8(5)));
    const NUM_BITS: u32 = 255;
    const ROOT_OF_UNITY: Self = HelioseleneField(Residue::new(&U256::from_be_hex(
        "7fffffffffffffffffffffffffffffffbf7f782cb7656b586eb6d2727927c79e",
    )));
    const ROOT_OF_UNITY_INV: Self = Self(Self::ROOT_OF_UNITY.0.invert().0);
    const S: u32 = 1;
    const TWO_INV: Self = HelioseleneField(ResidueType::new(&U256::from_u8(2)).invert().0);

    fn from_repr(bytes: Self::Repr) -> CtOption<Self> {
        let res = U256::from_le_slice(&bytes);
        CtOption::new(HelioseleneField(Residue::new(&res)), res.ct_lt(&MODULUS))
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
        repr.copy_from_slice(&MODULUS.to_le_bytes());
        repr.into()
    }
}
impl Sum<HelioseleneField> for HelioseleneField {
    fn sum<I: Iterator<Item = HelioseleneField>>(iter: I) -> HelioseleneField {
        let mut res = HelioseleneField::ZERO;
        for item in iter {
            res += item;
        }
        res
    }
}
impl<'a> Sum<&'a HelioseleneField> for HelioseleneField {
    fn sum<I: Iterator<Item = &'a HelioseleneField>>(iter: I) -> HelioseleneField {
        iter.cloned().sum()
    }
}
impl Product<HelioseleneField> for HelioseleneField {
    fn product<I: Iterator<Item = HelioseleneField>>(iter: I) -> HelioseleneField {
        let mut res = HelioseleneField::ONE;
        for item in iter {
            res *= item;
        }
        res
    }
}
impl<'a> Product<&'a HelioseleneField> for HelioseleneField {
    fn product<I: Iterator<Item = &'a HelioseleneField>>(iter: I) -> HelioseleneField {
        iter.cloned().product()
    }
}

impl HelioseleneField {
    /// Perform a wide reduction, presumably to get a non-biased Helioselene field element.
    pub fn wide_reduce(bytes: [u8; 64]) -> HelioseleneField {
        HelioseleneField(Residue::new(&reduce(U512::from_le_slice(bytes.as_ref()))))
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

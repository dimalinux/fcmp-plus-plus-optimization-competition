use core::{
    iter::Sum,
    ops::{Add, AddAssign, BitAnd, BitOr, Mul, MulAssign, Neg, Sub, SubAssign},
};

use group::{
    ff::{Field, PrimeField, PrimeFieldBits},
    prime::PrimeGroup,
    Group, GroupEncoding,
};
use rand_core::RngCore;
use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::{
    backend::u8_from_bool,
    fields::HelioseleneQ,
    u256::{MontyForm, U256},
    Field25519, HelioseleneField,
};

pub(crate) type MontyFormType = MontyForm<HelioseleneQ>;

pub(crate) const G_X: HelioseleneField = HelioseleneField(MontyForm::new(&U256::from_be_hex(
    "0000000000000000000000000000000000000000000000000000000000000001",
)));

pub(crate) const G_Y: HelioseleneField = HelioseleneField(MontyForm::new(&U256::from_be_hex(
    "7a19d927b85cca9257c93177455c825f938bb198c8f09b37741e0aa6a1d3fdd2",
)));

pub(crate) const B: HelioseleneField = HelioseleneField(MontyForm::new(&U256::from_be_hex(
    "70127713695876c17f51bba595ffe279f3944bdf06ae900e68de0983cb5a4558",
)));

/// B3 constant is the same as B + B + B
pub(crate) const B3: HelioseleneField = HelioseleneField(MontyForm::new(&U256::from_be_hex(
    "5037653a3c0964447df532f0c1ffa76e5bbdf343a540d97a5d2c77a66fbf40ca",
)));

fn recover_y(x: HelioseleneField) -> CtOption<HelioseleneField> {
    ((x.square() * x) - x - x - x + B).sqrt()
}
/// Point.
#[derive(Clone, Copy, Debug, Zeroize)]
pub struct SelenePoint {
    x: HelioseleneField,
    y: HelioseleneField,
    z: HelioseleneField,
}

const G: SelenePoint = SelenePoint {
    x: G_X,
    y: G_Y,
    z: HelioseleneField::ONE,
};
impl ConstantTimeEq for SelenePoint {
    fn ct_eq(&self, other: &Self) -> Choice {
        let x1 = MontyFormType::mul(&self.x.0, &other.z.0);
        let x2 = MontyFormType::mul(&other.x.0, &self.z.0);
        let y1 = MontyFormType::mul(&self.y.0, &other.z.0);
        let y2 = MontyFormType::mul(&other.y.0, &self.z.0);
        let both_x_zero = Choice::bitand(self.x.is_zero(), other.x.is_zero());
        let x_and_y_eq = Choice::bitand(x1.ct_eq(&x2), y1.ct_eq(&y2));
        Choice::bitor(both_x_zero, x_and_y_eq)
    }
}

impl PartialEq for SelenePoint {
    fn eq(&self, other: &Self) -> bool {
        // TODO: Does the contest use it? We could create a vartime eq method.
        self.ct_eq(other).into()
    }
}

impl Eq for SelenePoint {}

impl ConditionallySelectable for SelenePoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            x: HelioseleneField::conditional_select(&a.x, &b.x, choice),
            y: HelioseleneField::conditional_select(&a.y, &b.y, choice),
            z: HelioseleneField::conditional_select(&a.z, &b.z, choice),
        }
    }
}
impl Add for SelenePoint {
    type Output = Self;

    #[allow(non_snake_case)]
    fn add(self, other: Self) -> Self {
        let X1 = &self.x.0;
        let Y1 = &self.y.0;
        let Z1 = &self.z.0;
        let X2 = &other.x.0;
        let Y2 = &other.y.0;
        let Z2 = &other.z.0;
        const A: MontyFormType = MontyForm::neg(&MontyFormType::new(&U256::from_u64(3)));
        let t0 = MontyForm::mul(X1, X2);
        let t1 = MontyForm::mul(Y1, Y2);
        let t2 = MontyForm::mul(Z1, Z2);
        let t3 = MontyForm::mul(&MontyForm::add(X1, Y1), &MontyForm::add(X2, Y2));
        let t4 = MontyForm::add(&t0, &t1);
        let t3 = MontyForm::sub(&t3, &t4);
        let t4 = MontyForm::add(X1, Z1);
        let t5 = MontyForm::add(X2, Z2);
        let t4 = MontyForm::mul(&t4, &t5);
        let t5 = MontyForm::add(&t0, &t2);
        let t4 = MontyForm::sub(&t4, &t5);
        let t5 = MontyForm::add(Y1, Z1);
        let X3 = MontyForm::add(Y2, Z2);
        let t5 = MontyForm::mul(&t5, &X3);
        let X3 = MontyForm::add(&t1, &t2);
        let t5 = MontyForm::sub(&t5, &X3);
        let Z3 = MontyForm::mul(&A, &t4);
        let X3 = MontyForm::mul(&B3.0, &t2);
        let Z3 = MontyForm::add(&X3, &Z3);
        let X3 = MontyForm::sub(&t1, &Z3);
        let Z3 = MontyForm::add(&t1, &Z3);
        let Y3 = MontyForm::mul(&X3, &Z3);
        let t1 = MontyForm::add(&t0, &t0);
        let t1 = MontyForm::add(&t1, &t0);
        let t2 = MontyForm::mul(&A, &t2);
        let t4 = MontyForm::mul(&B3.0, &t4);
        let t1 = MontyForm::add(&t1, &t2);
        let t2 = MontyForm::sub(&t0, &t2);
        let t2 = MontyForm::mul(&A, &t2);
        let t4 = MontyForm::add(&t4, &t2);
        let t0 = MontyForm::mul(&t1, &t4);
        let Y3 = MontyForm::add(&Y3, &t0);
        let t0 = MontyForm::mul(&t5, &t4);
        let X3 = MontyForm::mul(&t3, &X3);
        let X3 = MontyForm::sub(&X3, &t0);
        let t0 = MontyForm::mul(&t3, &t1);
        let Z3 = MontyForm::mul(&t5, &Z3);
        let Z3 = MontyForm::add(&Z3, &t0);
        Self {
            x: HelioseleneField(X3),
            y: HelioseleneField(Y3),
            z: HelioseleneField(Z3),
        }
    }
}
impl AddAssign for SelenePoint {
    fn add_assign(&mut self, other: Self) {
        *self = Self::add(*self, other);
    }
}
impl Add<&Self> for SelenePoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self::add(self, *other)
    }
}
impl AddAssign<&Self> for SelenePoint {
    fn add_assign(&mut self, other: &Self) {
        *self = Self::add(*self, *other);
    }
}
impl Neg for SelenePoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: self.y.neg(),
            z: self.z,
        }
    }
}
impl Sub for SelenePoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::add(self, other.neg())
    }
}
impl SubAssign for SelenePoint {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::add(*self, other.neg());
    }
}
impl Sub<&Self> for SelenePoint {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self::add(self, other.neg())
    }
}
impl SubAssign<&Self> for SelenePoint {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self::add(*self, other.neg());
    }
}
impl Group for SelenePoint {
    type Scalar = Field25519;

    fn random(mut rng: impl RngCore) -> Self {
        loop {
            let mut bytes = HelioseleneField::random(&mut rng).to_repr();
            let mut_ref: &mut [u8] = bytes.as_mut();
            mut_ref[31] |= u8::try_from(rng.next_u32() % 2).unwrap() << 7;
            let opt = Self::from_bytes(&bytes);
            if opt.is_some().into() {
                return opt.unwrap();
            }
        }
    }

    fn identity() -> Self {
        Self {
            x: HelioseleneField::ZERO,
            y: HelioseleneField::ONE,
            z: HelioseleneField::ZERO,
        }
    }

    fn generator() -> Self {
        G
    }

    fn is_identity(&self) -> Choice {
        self.x.ct_eq(&HelioseleneField::ZERO)
    }

    #[allow(non_snake_case)]
    fn double(&self) -> Self {
        let X1 = self.x.0;
        let Y1 = self.y.0;
        let Z1 = self.z.0;
        let w = MontyFormType::mul(&MontyFormType::sub(&X1, &Z1), &MontyFormType::add(&X1, &Z1));
        let w = MontyFormType::add(&MontyFormType::add(&w, &w), &w);
        let s = MontyFormType::double(&MontyFormType::mul(&Y1, &Z1));
        let ss = MontyFormType::square(&s);
        let sss = MontyFormType::mul(&s, &ss);
        let R = MontyFormType::mul(&Y1, &s);
        let RR = R.square();
        let B_ = MontyFormType::mul(&X1, &R).double();
        let h = MontyFormType::sub(&w.square(), &B_.double());
        let X3 = MontyFormType::mul(&h, &s);
        let Y3 = MontyFormType::sub(
            &MontyFormType::mul(&w, &(MontyFormType::sub(&B_, &h))),
            &RR.double(),
        );
        let Z3 = sss;
        let res = Self {
            x: HelioseleneField(X3),
            y: HelioseleneField(Y3),
            z: HelioseleneField(Z3),
        };
        Self::conditional_select(&res, &Self::identity(), self.is_identity())
    }
}
impl Sum<Self> for SelenePoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::identity();
        for i in iter {
            res += i;
        }
        res
    }
}
impl<'a> Sum<&'a Self> for SelenePoint {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::sum(iter.copied())
    }
}
impl Mul<Field25519> for SelenePoint {
    type Output = Self;

    fn mul(self, mut other: Field25519) -> Self {
        let mut table = [Self::identity(); 16];
        table[1] = self;
        for i in 2..16 {
            table[i] = table[i - 1] + self;
        }
        let mut res = Self::identity();
        let mut bits = 0;
        for (i, mut bit) in other.to_le_bits().iter_mut().rev().enumerate() {
            bits <<= 1;
            let mut bit = u8_from_bool(&mut bit);
            bits |= bit;
            bit.zeroize();
            if ((i + 1) % 4) == 0 {
                if i != 3 {
                    for _ in 0..4 {
                        res = res.double();
                    }
                }
                let mut term = table[0];
                for (j, candidate) in table[1..].iter().enumerate() {
                    let j = j + 1;
                    term = Self::conditional_select(&term, candidate, usize::from(bits).ct_eq(&j));
                }
                res += term;
                bits = 0;
            }
        }
        other.zeroize();
        res
    }
}
impl MulAssign<Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: Field25519) {
        *self = *self * other;
    }
}
impl Mul<&Field25519> for SelenePoint {
    type Output = Self;

    fn mul(self, other: &Field25519) -> Self {
        self * *other
    }
}
impl MulAssign<&Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: &Field25519) {
        *self *= *other;
    }
}
impl GroupEncoding for SelenePoint {
    type Repr = <HelioseleneField as PrimeField>::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let sign = Choice::from(bytes[31] >> 7);
        let mut bytes = *bytes;
        let mut_ref: &mut [u8] = bytes.as_mut();
        mut_ref[31] &= !(1 << 7);
        HelioseleneField::from_repr(bytes).and_then(|x| {
            let is_identity = x.is_zero();
            let y = recover_y(x).map(|mut y| {
                y.conditional_negate(y.is_odd().ct_eq(&!sign));
                y
            });
            let y = CtOption::conditional_select(
                &y,
                &CtOption::new(HelioseleneField::ONE, 1.into()),
                is_identity,
            );
            let point = y.map(|y| Self {
                x,
                y,
                z: HelioseleneField::ONE,
            });
            let not_negative_zero = !(is_identity & sign);
            CtOption::conditional_select(
                &CtOption::new(Self::identity(), 0.into()),
                &point,
                not_negative_zero,
            )
        })
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes(&self) -> Self::Repr {
        let Some(z) = Option::<HelioseleneField>::from(self.z.invert()) else {
            return [0; 32];
        };
        let x = self.x * z;
        let y = self.y * z;
        let mut bytes = x.to_repr();
        let mut_ref: &mut [u8] = bytes.as_mut();
        let y_sign = u8::conditional_select(
            &y.is_odd().unwrap_u8(),
            &0,
            x.ct_eq(&HelioseleneField::ZERO),
        );
        mut_ref[31] |= y_sign << 7;
        bytes
    }
}
impl PrimeGroup for SelenePoint {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selene() {
        ff_group_tests::group::test_prime_group_bits::<_, SelenePoint>(&mut rand_core::OsRng);
    }

    #[test]
    fn generator_selene() {
        assert_eq!(G.x, G_X);
        assert_eq!(G.y, G_Y);
        assert_eq!(recover_y(G.x).unwrap(), G.y);
    }

    #[test]
    fn zero_x_is_invalid() {
        assert!(Option::<HelioseleneField>::from(recover_y(HelioseleneField::ZERO)).is_none());
    }

    #[test]
    fn b3_value() {
        assert_eq!(B + B + B, B3);
    }
}

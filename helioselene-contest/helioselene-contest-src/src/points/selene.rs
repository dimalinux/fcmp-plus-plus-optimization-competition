use core::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use group::{
    ff::{Field, PrimeField},
    prime::PrimeGroup,
    Group, GroupEncoding,
};
use rand_core::RngCore;
use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::{
    fields::HelioseleneQ,
    u256::{CtChoice, MontyForm, U256},
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
    // ((x.square() * x) - x - x - x + B).sqrt()
    let x = &x.0;
    let mut v = MontyFormType::square(x);
    v = MontyFormType::mul(&v, x);
    v = MontyFormType::sub(&v, x);
    v = MontyFormType::sub(&v, x);
    v = MontyFormType::sub(&v, x);
    v = MontyFormType::add(&v, &B.0);

    HelioseleneField(v).sqrt()
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
        let both_x_zero = self.x.ct_is_zero().and(other.x.ct_is_zero());
        let x_and_y_eq = x1.ct_eq(&x2).and(y1.ct_eq(&y2));
        both_x_zero.or(x_and_y_eq).into()
    }
}

impl PartialEq for SelenePoint {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).is_true_vartime()
    }
}

impl Eq for SelenePoint {}

impl ConditionallySelectable for SelenePoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self::ct_select(a, b, CtChoice::from(choice))
    }
}

impl SelenePoint {
    const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        Self {
            x: HelioseleneField::ct_select(&a.x, &b.x, c),
            y: HelioseleneField::ct_select(&a.y, &b.y, c),
            z: HelioseleneField::ct_select(&a.z, &b.z, c),
        }
    }

    const fn ct_eq(&self, other: &Self) -> CtChoice {
        let x1 = MontyFormType::mul(&self.x.0, &other.z.0);
        let x2 = MontyFormType::mul(&other.x.0, &self.z.0);
        let y1 = MontyFormType::mul(&self.y.0, &other.z.0);
        let y2 = MontyFormType::mul(&other.y.0, &self.z.0);
        let both_x_zero = self.x.ct_is_zero().and(other.x.ct_is_zero());
        let x_and_y_eq = x1.ct_eq(&x2).and(y1.ct_eq(&y2));
        both_x_zero.or(x_and_y_eq)
    }
}

impl SelenePoint {
    #[allow(non_snake_case)]
    const fn const_add(self, other: &Self) -> Self {
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

impl Add for SelenePoint {
    type Output = Self;

    #[allow(non_snake_case)]
    fn add(self, other: Self) -> Self {
        Self::const_add(self, &other)
    }
}

impl AddAssign for SelenePoint {
    fn add_assign(&mut self, other: Self) {
        *self = Self::const_add(*self, &other);
    }
}

impl Add<&Self> for SelenePoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self::const_add(self, other)
    }
}

impl AddAssign<&Self> for SelenePoint {
    fn add_assign(&mut self, other: &Self) {
        *self = Self::const_add(*self, other);
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
        Self::const_add(self, &other.neg())
    }
}

impl SubAssign for SelenePoint {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::const_add(*self, &other.neg());
    }
}

impl Sub<&Self> for SelenePoint {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self::const_add(self, &other.neg())
    }
}

impl SubAssign<&Self> for SelenePoint {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self::const_add(*self, &other.neg());
    }
}

const IDENTITY: SelenePoint = SelenePoint {
    x: HelioseleneField::ZERO,
    y: HelioseleneField::ONE,
    z: HelioseleneField::ZERO,
};

impl SelenePoint {
    #[allow(non_snake_case)]
    const fn const_double(&self) -> Self {
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

        let is_identity = self.x.ct_is_zero();

        Self {
            x: HelioseleneField(MontyFormType::ct_select(&X3, &IDENTITY.x.0, is_identity)),
            y: HelioseleneField(MontyFormType::ct_select(&Y3, &IDENTITY.y.0, is_identity)),
            z: HelioseleneField(MontyFormType::ct_select(&Z3, &IDENTITY.z.0, is_identity)),
        }
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
        self.x.ct_is_zero().into()
    }

    #[allow(non_snake_case)]
    fn double(&self) -> Self {
        Self::const_double(self)
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

impl SelenePoint {
    const fn const_mul(self, other: Field25519) -> Self {
        let mut table = [IDENTITY; 16];
        table[1] = self;
        let mut i = 2;
        while i < 16 {
            table[i] = Self::const_add(self, &table[i - 1]);
            i += 1;
        }

        let mut res = IDENTITY;
        let nibbles = other.0.retrieve().as_be_nibbles();
        let mut i = 0;

        #[allow(unused_assignments)]
        while i < 64 {
            let mut bits = nibbles[i];

            if i > 0 {
                res = res.const_double();
                res = res.const_double();
                res = res.const_double();
                res = res.const_double();
            }

            i += 1;
            let mut term = table[0];
            let mut j: usize = 1;

            while j < 16 {
                let c = CtChoice::from_u64_eq(bits as u64, j as u64);
                term = Self::ct_select(&term, &table[j], c);
                j += 1;
            }

            res = Self::const_add(res, &term);
            bits = 0;
        }

        // TODO: How to handle this in a const function?
        //nibbles.zeroize();
        //other.zeroize();
        res
    }
}

impl Mul<Field25519> for SelenePoint {
    type Output = Self;

    fn mul(self, other: Field25519) -> Self {
        Self::const_mul(self, other)
    }
}

impl MulAssign<Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: Field25519) {
        *self = Self::const_mul(*self, other);
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

    #[allow(clippy::cast_possible_truncation)]
    fn to_bytes(&self) -> Self::Repr {
        let Some(z) = Option::<HelioseleneField>::from(self.z.invert()) else {
            return [0; 32];
        };
        let x = self.x * z; // TODO: use const_mul directly
        let y = self.y * z;
        let mut bytes = x.to_repr();
        let mut_ref: &mut [u8] = bytes.as_mut();
        let y_lsb = y.0.retrieve().least_significant_bit();
        let x_is_zero = x.ct_is_zero();
        let y_sign = x_is_zero.select(y_lsb, 0);
        mut_ref[31] |= (y_sign << 7) as u8;

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

    #[test]
    fn test_const_mul() {
        let point = SelenePoint::generator();
        let scalar = Field25519::from(2_u64);
        let result = SelenePoint::const_mul(point, scalar);
        let expected = SelenePoint::const_add(point, &point); // 2 * G
        assert_eq!(result, expected);
    }
}

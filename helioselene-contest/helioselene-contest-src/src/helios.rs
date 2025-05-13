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
    dalek_ff_group::{Field25519, ResidueType},
    field::HelioseleneField,
    u256::{Residue, U256},
};

pub(crate) const G_X: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "0000000000000000000000000000000000000000000000000000000000000003",
)));
pub(crate) const G_Y: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "537b74d97ac0721cbd92668350205f0759003bddc586a5dcd243e639e3183ef4",
)));
const B: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "22e8c739b0ea70b8be94a76b3ebb7b3b043f6f384113bf3522b49ee1edd73ad4",
)));

const B3: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "68ba55ad12bf522a3bbdf641bc3271b10cbe4da8c33b3d9f681ddca5c985b07c",
)));

fn recover_y(x: Field25519) -> CtOption<Field25519> {
    ((x.square() * x) - x - x - x + B).sqrt()
}
/// Point.
#[derive(Clone, Copy, Debug, Zeroize)]
pub struct HeliosPoint {
    x: Field25519,
    y: Field25519,
    z: Field25519,
}

pub(crate) const G: HeliosPoint = HeliosPoint {
    x: G_X,
    y: G_Y,
    z: Field25519::ONE,
};
impl ConstantTimeEq for HeliosPoint {
    fn ct_eq(&self, other: &Self) -> Choice {
        let x1 = ResidueType::mul(&self.x.0, &other.z.0);
        let x2 = ResidueType::mul(&other.x.0, &self.z.0);
        let y1 = ResidueType::mul(&self.y.0, &other.z.0);
        let y2 = ResidueType::mul(&other.y.0, &self.z.0);
        let both_x_zero = Choice::bitand(self.x.is_zero(), other.x.is_zero());
        let x_and_y_eq = Choice::bitand(x1.ct_eq(&x2), y1.ct_eq(&y2));
        Choice::bitor(both_x_zero, x_and_y_eq)
    }
}

impl PartialEq for HeliosPoint {
    // TODO: Does the contest use it? We could create a vartime eq method.
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

impl Eq for HeliosPoint {}

impl ConditionallySelectable for HeliosPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            x: Field25519::conditional_select(&a.x, &b.x, choice),
            y: Field25519::conditional_select(&a.y, &b.y, choice),
            z: Field25519::conditional_select(&a.z, &b.z, choice),
        }
    }
}
impl Add for HeliosPoint {
    type Output = Self;

    #[allow(non_snake_case)]
    fn add(self, other: Self) -> Self {
        let X1 = &self.x.0;
        let Y1 = &self.y.0;
        let Z1 = &self.z.0;
        let X2 = &other.x.0;
        let Y2 = &other.y.0;
        let Z2 = &other.z.0;
        const A: ResidueType = Residue::neg(&ResidueType::new(&U256::from_u64(3)));
        let t0 = Residue::mul(X1, X2);
        let t1 = Residue::mul(Y1, Y2);
        let t2 = Residue::mul(Z1, Z2);
        let t3 = Residue::mul(&Residue::add(X1, Y1), &Residue::add(X2, Y2));
        let t4 = Residue::add(&t0, &t1);
        let t3 = Residue::sub(&t3, &t4);
        let t4 = Residue::add(X1, Z1);
        let t5 = Residue::add(X2, Z2);
        let t4 = Residue::mul(&t4, &t5);
        let t5 = Residue::add(&t0, &t2);
        let t4 = Residue::sub(&t4, &t5);
        let t5 = Residue::add(Y1, Z1);
        let X3 = Residue::add(Y2, Z2);
        let t5 = Residue::mul(&t5, &X3);
        let X3 = Residue::add(&t1, &t2);
        let t5 = Residue::sub(&t5, &X3);
        let Z3 = Residue::mul(&A, &t4);
        let X3 = Residue::mul(&B3.0, &t2);
        let Z3 = Residue::add(&X3, &Z3);
        let X3 = Residue::sub(&t1, &Z3);
        let Z3 = Residue::add(&t1, &Z3);
        let Y3 = Residue::mul(&X3, &Z3);
        let t1 = Residue::add(&t0, &t0);
        let t1 = Residue::add(&t1, &t0);
        let t2 = Residue::mul(&A, &t2);
        let t4 = Residue::mul(&B3.0, &t4);
        let t1 = Residue::add(&t1, &t2);
        let t2 = Residue::sub(&t0, &t2);
        let t2 = Residue::mul(&A, &t2);
        let t4 = Residue::add(&t4, &t2);
        let t0 = Residue::mul(&t1, &t4);
        let Y3 = Residue::add(&Y3, &t0);
        let t0 = Residue::mul(&t5, &t4);
        let X3 = Residue::mul(&t3, &X3);
        let X3 = Residue::sub(&X3, &t0);
        let t0 = Residue::mul(&t3, &t1);
        let Z3 = Residue::mul(&t5, &Z3);
        let Z3 = Residue::add(&Z3, &t0);
        Self {
            x: Field25519(X3),
            y: Field25519(Y3),
            z: Field25519(Z3),
        }
    }
}
impl AddAssign for HeliosPoint {
    fn add_assign(&mut self, other: Self) {
        *self = Self::add(*self, other);
    }
}
impl Add<&Self> for HeliosPoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self::add(self, *other)
    }
}
impl AddAssign<&Self> for HeliosPoint {
    fn add_assign(&mut self, other: &Self) {
        *self = Self::add(*self, *other);
    }
}
impl Neg for HeliosPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: self.y.neg(),
            z: self.z,
        }
    }
}
impl Sub for HeliosPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::add(self, other.neg())
    }
}
impl SubAssign for HeliosPoint {
    fn sub_assign(&mut self, other: Self) {
        *self = Self::add(*self, other.neg());
    }
}
impl Sub<&Self> for HeliosPoint {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self::add(self, other.neg())
    }
}
impl SubAssign<&Self> for HeliosPoint {
    fn sub_assign(&mut self, other: &Self) {
        *self = Self::add(*self, other.neg());
    }
}
impl Group for HeliosPoint {
    type Scalar = HelioseleneField;

    fn random(mut rng: impl RngCore) -> Self {
        loop {
            let mut bytes = Field25519::random(&mut rng).to_repr();
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
            x: Field25519::ZERO,
            y: Field25519::ONE,
            z: Field25519::ZERO,
        }
    }

    fn generator() -> Self {
        G
    }

    fn is_identity(&self) -> Choice {
        self.x.ct_eq(&Field25519::ZERO)
    }

    #[allow(non_snake_case)]
    fn double(&self) -> Self {
        let X1 = self.x.0;
        let Y1 = self.y.0;
        let Z1 = self.z.0;
        let w = ResidueType::mul(&ResidueType::sub(&X1, &Z1), &ResidueType::add(&X1, &Z1));
        let w = ResidueType::add(&ResidueType::add(&w, &w), &w);
        let s = ResidueType::double(&ResidueType::mul(&Y1, &Z1));
        let ss = ResidueType::square(&s);
        let sss = ResidueType::mul(&s, &ss);
        let R = ResidueType::mul(&Y1, &s);
        let RR = R.square();
        let B_ = ResidueType::mul(&X1, &R).double();
        let h = ResidueType::sub(&w.square(), &B_.double());
        let X3 = ResidueType::mul(&h, &s);
        let Y3 = ResidueType::sub(
            &ResidueType::mul(&w, &(ResidueType::sub(&B_, &h))),
            &RR.double(),
        );
        let Z3 = sss;
        let res = Self {
            x: Field25519(X3),
            y: Field25519(Y3),
            z: Field25519(Z3),
        };
        Self::conditional_select(&res, &Self::identity(), self.is_identity())
    }
}
impl Sum<Self> for HeliosPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::identity();
        for i in iter {
            res += i;
        }
        res
    }
}
impl<'a> Sum<&'a Self> for HeliosPoint {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::sum(iter.copied())
    }
}
impl Mul<HelioseleneField> for HeliosPoint {
    type Output = Self;

    fn mul(self, mut other: HelioseleneField) -> Self {
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
impl MulAssign<HelioseleneField> for HeliosPoint {
    fn mul_assign(&mut self, other: HelioseleneField) {
        *self = *self * other;
    }
}
impl Mul<&HelioseleneField> for HeliosPoint {
    type Output = Self;

    fn mul(self, other: &HelioseleneField) -> Self {
        self * *other
    }
}
impl MulAssign<&HelioseleneField> for HeliosPoint {
    fn mul_assign(&mut self, other: &HelioseleneField) {
        *self *= *other;
    }
}
impl GroupEncoding for HeliosPoint {
    type Repr = <Field25519 as PrimeField>::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let sign = Choice::from(bytes[31] >> 7);
        let mut bytes = *bytes;
        let mut_ref: &mut [u8] = bytes.as_mut();
        mut_ref[31] &= !(1 << 7);
        Field25519::from_repr(bytes).and_then(|x| {
            let is_identity = x.is_zero();
            let y = recover_y(x).map(|mut y| {
                y.conditional_negate(y.is_odd().ct_eq(&!sign));
                y
            });
            let y = CtOption::conditional_select(
                &y,
                &CtOption::new(Field25519::ONE, 1.into()),
                is_identity,
            );
            let point = y.map(|y| Self {
                x,
                y,
                z: Field25519::ONE,
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
        let Some(z) = Option::<Field25519>::from(self.z.invert()) else {
            return [0; 32];
        };
        let x = self.x * z;
        let y = self.y * z;
        let mut bytes = x.to_repr();
        let mut_ref: &mut [u8] = bytes.as_mut();
        let y_sign =
            u8::conditional_select(&y.is_odd().unwrap_u8(), &0, x.ct_eq(&Field25519::ZERO));
        mut_ref[31] |= y_sign << 7;
        bytes
    }
}
impl PrimeGroup for HeliosPoint {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helios() {
        ff_group_tests::group::test_prime_group_bits::<_, HeliosPoint>(&mut rand_core::OsRng);
    }

    #[test]
    fn generator_helios() {
        assert_eq!(G.x, G_X);
        assert_eq!(G.y, G_Y);
        assert_eq!(recover_y(G.x).unwrap(), -G.y);
    }

    #[test]
    fn zero_x_is_invalid() {
        assert!(Option::<Field25519>::from(recover_y(Field25519::ZERO)).is_none());
    }

    #[test]
    fn b3_value() {
        assert_eq!(B + B + B, B3);
    }
}

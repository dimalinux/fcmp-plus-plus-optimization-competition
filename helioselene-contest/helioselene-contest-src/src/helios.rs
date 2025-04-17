use core::{
    iter::Sum,
    ops::{Add, AddAssign, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crypto_bigint::{modular::constant_mod::Residue, U256};
use dalek_ff_group::FieldElement as Field25519;
use group::{
    ff::{Field, PrimeField, PrimeFieldBits},
    prime::PrimeGroup,
    Group, GroupEncoding,
};
use rand_core::RngCore;
use subtle::{Choice, ConditionallyNegatable, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::{backend::u8_from_bool, field::HelioseleneField};

pub(crate) const G_X: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "0000000000000000000000000000000000000000000000000000000000000003",
)));
pub(crate) const G_Y: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "537b74d97ac0721cbd92668350205f0759003bddc586a5dcd243e639e3183ef4",
)));
const B: Field25519 = Field25519(Residue::new(&U256::from_be_hex(
    "22e8c739b0ea70b8be94a76b3ebb7b3b043f6f384113bf3522b49ee1edd73ad4",
)));
fn recover_y(x: Field25519) -> CtOption<Field25519> {
    ((x.square() * x) - x - x - x + B).sqrt()
}
/// Point.
#[derive(Clone, Copy, Debug, Zeroize)]
#[repr(C)]
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
        let x1 = self.x * other.z;
        let x2 = other.x * self.z;
        let y1 = self.y * other.z;
        let y2 = other.y * self.z;
        (self.x.is_zero() & other.x.is_zero()) | (x1.ct_eq(&x2) & y1.ct_eq(&y2))
    }
}
impl PartialEq for HeliosPoint {
    fn eq(&self, other: &HeliosPoint) -> bool {
        self.ct_eq(other).into()
    }
}
impl Eq for HeliosPoint {}
impl ConditionallySelectable for HeliosPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        HeliosPoint {
            x: Field25519::conditional_select(&a.x, &b.x, choice),
            y: Field25519::conditional_select(&a.y, &b.y, choice),
            z: Field25519::conditional_select(&a.z, &b.z, choice),
        }
    }
}
impl Add for HeliosPoint {
    type Output = HeliosPoint;

    #[allow(non_snake_case)]
    fn add(self, other: Self) -> Self {
        let b3 = B + B + B;
        let X1 = self.x;
        let Y1 = self.y;
        let Z1 = self.z;
        let X2 = other.x;
        let Y2 = other.y;
        let Z2 = other.z;
        let a = -Field25519::from(3u64);
        let t0 = X1 * X2;
        let t1 = Y1 * Y2;
        let t2 = Z1 * Z2;
        let t3 = X1 + Y1;
        let t4 = X2 + Y2;
        let t3 = t3 * t4;
        let t4 = t0 + t1;
        let t3 = t3 - t4;
        let t4 = X1 + Z1;
        let t5 = X2 + Z2;
        let t4 = t4 * t5;
        let t5 = t0 + t2;
        let t4 = t4 - t5;
        let t5 = Y1 + Z1;
        let X3 = Y2 + Z2;
        let t5 = t5 * X3;
        let X3 = t1 + t2;
        let t5 = t5 - X3;
        let Z3 = a * t4;
        let X3 = b3 * t2;
        let Z3 = X3 + Z3;
        let X3 = t1 - Z3;
        let Z3 = t1 + Z3;
        let Y3 = X3 * Z3;
        let t1 = t0 + t0;
        let t1 = t1 + t0;
        let t2 = a * t2;
        let t4 = b3 * t4;
        let t1 = t1 + t2;
        let t2 = t0 - t2;
        let t2 = a * t2;
        let t4 = t4 + t2;
        let t0 = t1 * t4;
        let Y3 = Y3 + t0;
        let t0 = t5 * t4;
        let X3 = t3 * X3;
        let X3 = X3 - t0;
        let t0 = t3 * t1;
        let Z3 = t5 * Z3;
        let Z3 = Z3 + t0;
        HeliosPoint {
            x: X3,
            y: Y3,
            z: Z3,
        }
    }
}
impl AddAssign for HeliosPoint {
    fn add_assign(&mut self, other: HeliosPoint) {
        *self = *self + other;
    }
}
impl Add<&HeliosPoint> for HeliosPoint {
    type Output = HeliosPoint;

    fn add(self, other: &HeliosPoint) -> HeliosPoint {
        self + *other
    }
}
impl AddAssign<&HeliosPoint> for HeliosPoint {
    fn add_assign(&mut self, other: &HeliosPoint) {
        *self += *other;
    }
}
impl Neg for HeliosPoint {
    type Output = HeliosPoint;

    fn neg(self) -> Self {
        HeliosPoint {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}
impl Sub for HeliosPoint {
    type Output = HeliosPoint;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}
impl SubAssign for HeliosPoint {
    fn sub_assign(&mut self, other: HeliosPoint) {
        *self = *self - other;
    }
}
impl Sub<&HeliosPoint> for HeliosPoint {
    type Output = HeliosPoint;

    fn sub(self, other: &HeliosPoint) -> HeliosPoint {
        self - *other
    }
}
impl SubAssign<&HeliosPoint> for HeliosPoint {
    fn sub_assign(&mut self, other: &HeliosPoint) {
        *self -= *other;
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
        HeliosPoint {
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
        let X1 = self.x;
        let Y1 = self.y;
        let Z1 = self.z;
        let w = (X1 - Z1) * (X1 + Z1);
        let w = w.double() + w;
        let s = (Y1 * Z1).double();
        let ss = s.square();
        let sss = s * ss;
        let R = Y1 * s;
        let RR = R.square();
        let B_ = (X1 * R).double();
        let h = w.square() - B_.double();
        let X3 = h * s;
        let Y3 = w * (B_ - h) - RR.double();
        let Z3 = sss;
        let res = Self {
            x: X3,
            y: Y3,
            z: Z3,
        };
        Self::conditional_select(&res, &Self::identity(), self.is_identity())
    }
}
impl Sum<HeliosPoint> for HeliosPoint {
    fn sum<I: Iterator<Item = HeliosPoint>>(iter: I) -> HeliosPoint {
        let mut res = Self::identity();
        for i in iter {
            res += i;
        }
        res
    }
}
impl<'a> Sum<&'a HeliosPoint> for HeliosPoint {
    fn sum<I: Iterator<Item = &'a HeliosPoint>>(iter: I) -> HeliosPoint {
        HeliosPoint::sum(iter.cloned())
    }
}
impl Mul<HelioseleneField> for HeliosPoint {
    type Output = HeliosPoint;

    fn mul(self, mut other: HelioseleneField) -> HeliosPoint {
        let mut table = [HeliosPoint::identity(); 16];
        table[1] = self;
        for i in 2..16 {
            table[i] = table[i - 1] + self;
        }
        let mut res = Self::identity();
        let mut bits = 0;
        for (i, mut bit) in other.to_le_bits().iter_mut().rev().enumerate() {
            bits <<= 1;
            let mut bit = u8_from_bool(bit.deref_mut());
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
    type Output = HeliosPoint;

    fn mul(self, other: &HelioseleneField) -> HeliosPoint {
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
            let point = y.map(|y| HeliosPoint {
                x,
                y,
                z: Field25519::ONE,
            });
            let not_negative_zero = !(is_identity & sign);
            CtOption::conditional_select(
                &CtOption::new(HeliosPoint::identity(), 0.into()),
                &point,
                not_negative_zero,
            )
        })
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        HeliosPoint::from_bytes(bytes)
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
impl ec_divisors::DivisorCurve for HeliosPoint {
    type FieldElement = Field25519;

    fn a() -> Self::FieldElement {
        -Field25519::from(3u64)
    }

    fn b() -> Self::FieldElement {
        B
    }

    fn to_xy(point: Self) -> Option<(Self::FieldElement, Self::FieldElement)> {
        let z: Self::FieldElement = Option::from(point.z.invert())?;
        Some((point.x * z, point.y * z))
    }
}

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
}

use core::{
    iter::Sum,
    ops::{Add, AddAssign, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign},
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
    bigint::{Residue, U256},
    dalek_ff_group::Field25519,
    field::HelioseleneField,
};

pub(crate) const G_X: HelioseleneField = HelioseleneField(Residue::new(&U256::from_be_hex(
    "0000000000000000000000000000000000000000000000000000000000000001",
)));

pub(crate) const G_Y: HelioseleneField = HelioseleneField(Residue::new(&U256::from_be_hex(
    "7a19d927b85cca9257c93177455c825f938bb198c8f09b37741e0aa6a1d3fdd2",
)));

pub(crate) const B: HelioseleneField = HelioseleneField(Residue::new(&U256::from_be_hex(
    "70127713695876c17f51bba595ffe279f3944bdf06ae900e68de0983cb5a4558",
)));
fn recover_y(x: HelioseleneField) -> CtOption<HelioseleneField> {
    ((x.square() * x) - x - x - x + B).sqrt()
}
/// Point.
#[derive(Clone, Copy, Debug, Zeroize)]
#[repr(C)]
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
        let x1 = self.x * other.z;
        let x2 = other.x * self.z;
        let y1 = self.y * other.z;
        let y2 = other.y * self.z;
        (self.x.is_zero() & other.x.is_zero()) | (x1.ct_eq(&x2) & y1.ct_eq(&y2))
    }
}
impl PartialEq for SelenePoint {
    fn eq(&self, other: &SelenePoint) -> bool {
        self.ct_eq(other).into()
    }
}
impl Eq for SelenePoint {}
impl ConditionallySelectable for SelenePoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        SelenePoint {
            x: HelioseleneField::conditional_select(&a.x, &b.x, choice),
            y: HelioseleneField::conditional_select(&a.y, &b.y, choice),
            z: HelioseleneField::conditional_select(&a.z, &b.z, choice),
        }
    }
}
impl Add for SelenePoint {
    type Output = SelenePoint;

    #[allow(non_snake_case)]
    fn add(self, other: Self) -> Self {
        let b3 = B + B + B;
        let X1 = self.x;
        let Y1 = self.y;
        let Z1 = self.z;
        let X2 = other.x;
        let Y2 = other.y;
        let Z2 = other.z;
        let a = -HelioseleneField::from(3u64);
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
        SelenePoint {
            x: X3,
            y: Y3,
            z: Z3,
        }
    }
}
impl AddAssign for SelenePoint {
    fn add_assign(&mut self, other: SelenePoint) {
        *self = *self + other;
    }
}
impl Add<&SelenePoint> for SelenePoint {
    type Output = SelenePoint;

    fn add(self, other: &SelenePoint) -> SelenePoint {
        self + *other
    }
}
impl AddAssign<&SelenePoint> for SelenePoint {
    fn add_assign(&mut self, other: &SelenePoint) {
        *self += *other;
    }
}
impl Neg for SelenePoint {
    type Output = SelenePoint;

    fn neg(self) -> Self {
        SelenePoint {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}
impl Sub for SelenePoint {
    type Output = SelenePoint;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: Self) -> Self {
        self + other.neg()
    }
}
impl SubAssign for SelenePoint {
    fn sub_assign(&mut self, other: SelenePoint) {
        *self = *self - other;
    }
}
impl Sub<&SelenePoint> for SelenePoint {
    type Output = SelenePoint;

    fn sub(self, other: &SelenePoint) -> SelenePoint {
        self - *other
    }
}
impl SubAssign<&SelenePoint> for SelenePoint {
    fn sub_assign(&mut self, other: &SelenePoint) {
        *self -= *other;
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
        SelenePoint {
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
impl Sum<SelenePoint> for SelenePoint {
    fn sum<I: Iterator<Item = SelenePoint>>(iter: I) -> SelenePoint {
        let mut res = Self::identity();
        for i in iter {
            res += i;
        }
        res
    }
}
impl<'a> Sum<&'a SelenePoint> for SelenePoint {
    fn sum<I: Iterator<Item = &'a SelenePoint>>(iter: I) -> SelenePoint {
        SelenePoint::sum(iter.cloned())
    }
}
impl Mul<Field25519> for SelenePoint {
    type Output = SelenePoint;

    fn mul(self, mut other: Field25519) -> SelenePoint {
        let mut table = [SelenePoint::identity(); 16];
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
impl MulAssign<Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: Field25519) {
        *self = *self * other;
    }
}
impl Mul<&Field25519> for SelenePoint {
    type Output = SelenePoint;

    fn mul(self, other: &Field25519) -> SelenePoint {
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
            let point = y.map(|y| SelenePoint {
                x,
                y,
                z: HelioseleneField::ONE,
            });
            let not_negative_zero = !(is_identity & sign);
            CtOption::conditional_select(
                &CtOption::new(SelenePoint::identity(), 0.into()),
                &point,
                not_negative_zero,
            )
        })
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        SelenePoint::from_bytes(bytes)
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
}

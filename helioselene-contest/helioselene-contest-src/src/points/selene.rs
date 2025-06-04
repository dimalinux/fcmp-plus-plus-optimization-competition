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
    fields::HelioseleneParams,
    points::point::{Point, PointParams},
    u256::{CtChoice, MontyForm, U256},
    Field25519, HelioseleneField,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SelenePointParams;

impl PointParams<HelioseleneParams> for SelenePointParams {
    const B: MontyForm<HelioseleneParams> = MontyForm::new(&U256::from_be_hex(
        "70127713695876c17f51bba595ffe279f3944bdf06ae900e68de0983cb5a4558",
    ));
    const G_X: MontyForm<HelioseleneParams> = MontyForm::new(&U256::from_u64(1));
    const G_Y: MontyForm<HelioseleneParams> = MontyForm::new(&U256::from_be_hex(
        "7a19d927b85cca9257c93177455c825f938bb198c8f09b37741e0aa6a1d3fdd2",
    ));
}

/// Point.
#[derive(Clone, Copy, Debug, Zeroize)]
pub struct SelenePoint(Point<HelioseleneParams, SelenePointParams>);

fn recover_y(x: HelioseleneField) -> CtOption<HelioseleneField> {
    // ((x.square() * x) - x - x - x + B).sqrt()
    let x = &x.0;
    let mut v = MontyForm::square(x);
    v = MontyForm::mul(&v, x);
    v = MontyForm::sub(&v, x);
    v = MontyForm::sub(&v, x);
    v = MontyForm::sub(&v, x);
    v = MontyForm::add(&v, &SelenePointParams::B);

    HelioseleneField(v).sqrt()
}

impl SelenePoint {
    pub const fn new(x: HelioseleneField, y: HelioseleneField, z: HelioseleneField) -> Self {
        Self(Point::new(x.0, y.0, z.0))
    }

    pub const fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        Self(Point::ct_select(&a.0, &b.0, c))
    }

    const fn ct_eq(&self, other: &Self) -> CtChoice {
        self.0.ct_eq(&other.0)
    }
}

impl ConstantTimeEq for SelenePoint {
    fn ct_eq(&self, other: &Self) -> Choice {
        Self::ct_eq(self, other).into()
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

impl Add for SelenePoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.const_add(&other.0))
    }
}

impl AddAssign for SelenePoint {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.const_add(&other.0);
    }
}

impl Add<&Self> for SelenePoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self(self.0.const_add(&other.0))
    }
}

impl AddAssign<&Self> for SelenePoint {
    fn add_assign(&mut self, other: &Self) {
        self.0 = self.0.const_add(&other.0);
    }
}

impl Neg for SelenePoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.const_neg())
    }
}

impl Sub for SelenePoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.const_add(&other.0.const_neg()))
    }
}

impl SubAssign for SelenePoint {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.const_sub(&other.0);
    }
}

impl Sub<&Self> for SelenePoint {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self(self.0.const_sub(&other.0))
    }
}

impl SubAssign<&Self> for SelenePoint {
    fn sub_assign(&mut self, other: &Self) {
        self.0 = self.0.const_sub(&other.0);
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
        Self(Point::IDENTITY)
    }

    fn generator() -> Self {
        Self(Point::G)
    }

    fn is_identity(&self) -> Choice {
        self.0.is_identity().into()
    }

    #[inline]
    fn double(&self) -> Self {
        Self(self.0.const_double())
    }
}

impl Sum<Self> for SelenePoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Point::IDENTITY;
        for i in iter {
            res = res.const_add(&i.0);
        }
        Self(res)
    }
}

impl<'a> Sum<&'a Self> for SelenePoint {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::sum(iter.copied())
    }
}

impl Mul<Field25519> for SelenePoint {
    type Output = Self;

    fn mul(self, other: Field25519) -> Self {
        Self(self.0.const_mul(other.0))
    }
}

impl MulAssign<Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: Field25519) {
        *self = Self(self.0.const_mul(other.0));
    }
}

impl Mul<&Field25519> for SelenePoint {
    type Output = Self;

    fn mul(self, other: &Field25519) -> Self {
        Self(self.0.const_mul(other.0))
    }
}

impl MulAssign<&Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: &Field25519) {
        self.0 = self.0.const_mul(other.0);
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
            let y = if let Some(mut y) = Option::<HelioseleneField>::from(recover_y(x)) {
                y.conditional_negate(y.is_odd().ct_eq(&!sign));
                CtOption::new(y, 1.into())
            } else {
                CtOption::new(HelioseleneField::ZERO, 0.into())
            };
            let y = CtOption::conditional_select(
                &y,
                &CtOption::new(HelioseleneField::ONE, 1.into()),
                is_identity,
            );
            let point = y.map(|y| Self::new(x, y, HelioseleneField::ONE));
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
        Self::to_bytes(self)
    }
}

impl PrimeGroup for SelenePoint {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Field25519;

    #[test]
    fn test_helios() {
        ff_group_tests::group::test_prime_group_bits::<_, SelenePoint>(&mut rand_core::OsRng);
    }

    #[test]
    fn generator_selene() {
        const G: SelenePoint = SelenePoint(Point::G);
        assert_eq!(recover_y(HelioseleneField(G.0.x)).unwrap().0, G.0.y);
    }

    #[test]
    fn zero_x_is_invalid() {
        assert!(Option::<HelioseleneField>::from(recover_y(HelioseleneField::ZERO)).is_none());
    }

    #[test]
    fn test_const_mul() {
        let point = SelenePoint::generator();
        let scalar = Field25519::from(2_u64);
        let result = point.mul(scalar); // 2 * G
        let expected = point.add(&point); // G + G
        assert_eq!(result, expected);
    }
}

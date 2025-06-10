use core::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use group::{ff::PrimeField, prime::PrimeGroup, Group, GroupEncoding};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
use zeroize::Zeroize;

use crate::{
    fields::Field25519Params,
    points::point::{Point, PointParams},
    u256::{CtChoice, MontyForm, U256},
    Field25519, HelioseleneField,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct HeliosPointParams;

impl PointParams<Field25519Params> for HeliosPointParams {
    const B: MontyForm<Field25519Params> = MontyForm::new(&U256::from_be_hex(
        "22e8c739b0ea70b8be94a76b3ebb7b3b043f6f384113bf3522b49ee1edd73ad4",
    ));
    const G_X: MontyForm<Field25519Params> = MontyForm::new(&U256::from_u64(3));
    const G_Y: MontyForm<Field25519Params> = MontyForm::new(&U256::from_be_hex(
        "537b74d97ac0721cbd92668350205f0759003bddc586a5dcd243e639e3183ef4",
    ));
}

#[derive(Clone, Copy, Debug, Zeroize)]
pub struct HeliosPoint(Point<Field25519Params, HeliosPointParams>);

impl HeliosPoint {
    pub const fn new(x: Field25519, y: Field25519, z: Field25519) -> Self {
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

impl ConstantTimeEq for HeliosPoint {
    fn ct_eq(&self, other: &Self) -> Choice {
        Self::ct_eq(self, other).into()
    }
}

impl PartialEq for HeliosPoint {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).is_true_vartime()
    }
}

impl Eq for HeliosPoint {}

impl ConditionallySelectable for HeliosPoint {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self::ct_select(a, b, CtChoice::from(choice))
    }
}

impl Add for HeliosPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.add(&other.0))
    }
}

impl AddAssign for HeliosPoint {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(&other.0);
    }
}

impl Add<&Self> for HeliosPoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self(self.0.add(&other.0))
    }
}

impl AddAssign<&Self> for HeliosPoint {
    fn add_assign(&mut self, other: &Self) {
        self.0 = self.0.add(&other.0);
    }
}

impl Neg for HeliosPoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Sub for HeliosPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.add(&other.0.neg()))
    }
}

impl SubAssign for HeliosPoint {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub(&other.0);
    }
}

impl Sub<&Self> for HeliosPoint {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self(self.0.sub(&other.0))
    }
}

impl SubAssign<&Self> for HeliosPoint {
    fn sub_assign(&mut self, other: &Self) {
        self.0 = self.0.sub(&other.0);
    }
}

impl Group for HeliosPoint {
    type Scalar = HelioseleneField;

    fn random(mut rng: impl RngCore) -> Self {
        Self(Point::random(&mut rng))
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

    fn double(&self) -> Self {
        Self(self.0.double())
    }
}

impl Sum<Self> for HeliosPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Point::IDENTITY;
        for i in iter {
            res = res.add(&i.0);
        }
        Self(res)
    }
}

impl<'a> Sum<&'a Self> for HeliosPoint {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::sum(iter.copied())
    }
}

impl Mul<HelioseleneField> for HeliosPoint {
    type Output = Self;

    fn mul(self, other: HelioseleneField) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<HelioseleneField> for HeliosPoint {
    fn mul_assign(&mut self, other: HelioseleneField) {
        *self = Self(self.0.mul(other.0));
    }
}

impl Mul<&HelioseleneField> for HeliosPoint {
    type Output = Self;

    fn mul(self, other: &HelioseleneField) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<&HelioseleneField> for HeliosPoint {
    fn mul_assign(&mut self, other: &HelioseleneField) {
        self.0 = self.0.mul(other.0);
    }
}

impl GroupEncoding for HeliosPoint {
    type Repr = <Field25519 as PrimeField>::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let (pt, is_valid) = Point::from_bytes(bytes);
        CtOption::new(Self(pt), is_valid.into())
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        Self::from_bytes(bytes)
    }

    fn to_bytes(&self) -> Self::Repr {
        self.0.to_bytes()
    }
}

impl PrimeGroup for HeliosPoint {}

#[cfg(test)]
mod tests {
    use ff::Field;

    use super::*;
    use crate::HelioseleneField;

    #[test]
    fn test_helios() {
        ff_group_tests::group::test_prime_group_bits::<_, HeliosPoint>(&mut rand_core::OsRng);
    }

    #[test]
    fn generator_helios() {
        const G: HeliosPoint = HeliosPoint(Point::G);
        let (res, c) = Point::<Field25519Params, HeliosPointParams>::recover_y(G.0.x);
        assert!(c.is_true_vartime());
        assert_eq!(res, G.0.y.neg());
    }

    #[test]
    fn zero_x_is_invalid() {
        let (_, c) = Point::<Field25519Params, HeliosPointParams>::recover_y(Field25519::ZERO.0);
        assert!(!c.is_true_vartime());
    }

    #[test]
    fn test_mul() {
        let point = HeliosPoint::generator();
        let scalar = HelioseleneField::from(2_u64);
        let result = point.mul(scalar); // 2 * G
        let expected = point.add(&point); // G + G
        assert_eq!(result, expected);
    }
}

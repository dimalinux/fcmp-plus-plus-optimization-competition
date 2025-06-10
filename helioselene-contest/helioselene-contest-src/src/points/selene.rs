use core::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use group::{ff::PrimeField, prime::PrimeGroup, Group, GroupEncoding};
use rand_core::RngCore;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
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
#[derive(Clone, Copy, Debug, Default, Zeroize)]
pub struct SelenePoint(Point<HelioseleneParams, SelenePointParams>);

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
        Self(self.0.add(&other.0))
    }
}

impl AddAssign for SelenePoint {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(&other.0);
    }
}

impl Add<&Self> for SelenePoint {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self(self.0.add(&other.0))
    }
}

impl AddAssign<&Self> for SelenePoint {
    fn add_assign(&mut self, other: &Self) {
        self.0 = self.0.add(&other.0);
    }
}

impl Neg for SelenePoint {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Sub for SelenePoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.add(&other.0.neg()))
    }
}

impl SubAssign for SelenePoint {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub(&other.0);
    }
}

impl Sub<&Self> for SelenePoint {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Self(self.0.sub(&other.0))
    }
}

impl SubAssign<&Self> for SelenePoint {
    fn sub_assign(&mut self, other: &Self) {
        self.0 = self.0.sub(&other.0);
    }
}

impl Group for SelenePoint {
    type Scalar = Field25519;

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

    #[inline]
    fn double(&self) -> Self {
        Self(self.0.double())
    }
}

impl Sum<Self> for SelenePoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Point::IDENTITY;
        for i in iter {
            res = res.add(&i.0);
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
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: Field25519) {
        *self = Self(self.0.mul(other.0));
    }
}

impl Mul<&Field25519> for SelenePoint {
    type Output = Self;

    fn mul(self, other: &Field25519) -> Self {
        Self(self.0.mul(other.0))
    }
}

impl MulAssign<&Field25519> for SelenePoint {
    fn mul_assign(&mut self, other: &Field25519) {
        self.0 = self.0.mul(other.0);
    }
}

impl GroupEncoding for SelenePoint {
    type Repr = <HelioseleneField as PrimeField>::Repr;

    fn from_bytes(bytes: &Self::Repr) -> CtOption<Self> {
        let (pt, is_valid) = Point::from_bytes(bytes);
        CtOption::new(Self(pt), is_valid.into())
    }

    fn from_bytes_unchecked(bytes: &Self::Repr) -> CtOption<Self> {
        GroupEncoding::from_bytes(bytes)
    }

    fn to_bytes(&self) -> Self::Repr {
        self.0.to_bytes()
    }
}

impl PrimeGroup for SelenePoint {}

#[cfg(test)]
mod tests {
    use ff::Field;

    use super::*;
    use crate::Field25519;

    #[test]
    fn test_helios() {
        ff_group_tests::group::test_prime_group_bits::<_, SelenePoint>(&mut rand_core::OsRng);
    }

    #[test]
    fn generator_selene() {
        const G: SelenePoint = SelenePoint(Point::G);
        let (res, c) = Point::<HelioseleneParams, SelenePointParams>::recover_y(G.0.x);
        assert!(c.is_true_vartime());
        assert_eq!(res, G.0.y);
    }

    #[test]
    fn zero_x_is_invalid() {
        let (_, c) =
            Point::<HelioseleneParams, SelenePointParams>::recover_y(HelioseleneField::ZERO.0);
        assert!(!c.is_true_vartime());
    }

    #[test]
    fn test_mul() {
        let point = SelenePoint::generator();
        let scalar = Field25519::from(2_u64);
        let result = point.mul(scalar); // 2 * G
        let expected = point.add(&point); // G + G
        assert_eq!(result, expected);
    }

    #[test]
    fn selene_from_bytes() {
        struct TC {
            is_valid: bool,
            input: &'static str, // big endian hex
        }

        let tests: &[TC] = &[
            TC {
                is_valid: false, // larger than modulus
                input: "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            },
            TC {
                is_valid: false, // negative zero
                input: "8000000000000000000000000000000000000000000000000000000000000000",
            },
            TC {
                is_valid: false, // fails sqrt when recovering y
                input: "9e272cc20ef77ba18afd1b85c5fdd47d78322979704face1cd7bd24eafe103d1",
            },
            TC {
                is_valid: true,
                input: "6cc9fe90bc7a09a47f50535df1463e5b0643adfcce68c68b00d35781c02d9e1f",
            },
            TC {
                is_valid: true, // generator
                input: "0000000000000000000000000000000000000000000000000000000000000001",
            },
            TC {
                is_valid: true, // identity point
                input: "0000000000000000000000000000000000000000000000000000000000000000",
            },
        ];

        for tc in tests {
            let bytes: [u8; 32] = U256::from_be_hex(tc.input).to_le_bytes();
            let point = SelenePoint::from_bytes(&bytes);
            assert_eq!(bool::from(point.is_some()), tc.is_valid, "{}", tc.input);
            if tc.is_valid {
                // verify that we serialize back to the original input
                let mut point_bytes = point.unwrap().to_bytes();
                point_bytes.reverse(); // back to big-endian
                assert_eq!(hex::encode(point_bytes), tc.input);
            }
        }
    }
}

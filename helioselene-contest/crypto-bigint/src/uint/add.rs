//! [`Uint`] addition operations.

use crate::{CtChoice, Limb, Uint, Wrapping};
use core::ops::{Add, AddAssign};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `a + b + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn adc(&self, rhs: &Self, mut carry: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, c) = self.limbs[i].adc(rhs.limbs[i], carry);
            limbs[i] = w;
            carry = c;
            i += 1;
        }

        (Self { limbs }, carry)
    }

    /// Perform saturating addition, returning `MAX` on overflow.
    pub const fn saturating_add(&self, rhs: &Self) -> Self {
        let (res, overflow) = self.adc(rhs, Limb::ZERO);
        Self::ct_select(&res, &Self::MAX, CtChoice::from_lsb(overflow.0))
    }

    /// Perform wrapping addition, discarding overflow.
    pub const fn wrapping_add(&self, rhs: &Self) -> Self {
        self.adc(rhs, Limb::ZERO).0
    }

    /// Perform wrapping addition, returning the truthy value as the second element of the tuple
    /// if an overflow has occurred.
    pub(crate) const fn conditional_wrapping_add(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Uint::ct_select(&Uint::ZERO, rhs, choice);
        let (sum, carry) = self.adc(&actual_rhs, Limb::ZERO);
        (sum, CtChoice::from_lsb(carry.0))
    }
}

impl<const LIMBS: usize> Add for Wrapping<Uint<LIMBS>> {
    type Output = Self;

    fn add(self, rhs: Self) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_add(&rhs.0))
    }
}

impl<const LIMBS: usize> Add<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn add(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_add(&rhs.0))
    }
}

impl<const LIMBS: usize> Add<Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn add(self, rhs: Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_add(&rhs.0))
    }
}

impl<const LIMBS: usize> Add<&Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn add(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_add(&rhs.0))
    }
}

impl<const LIMBS: usize> AddAssign for Wrapping<Uint<LIMBS>> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<const LIMBS: usize> AddAssign<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other;
    }
}



#[cfg(test)]
mod tests {
    use crate::{Limb, U128};

    #[test]
    fn adc_no_carry() {
        let (res, carry) = U128::ZERO.adc(&U128::ONE, Limb::ZERO);
        assert_eq!(res, U128::ONE);
        assert_eq!(carry, Limb::ZERO);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = U128::MAX.adc(&U128::ONE, Limb::ZERO);
        assert_eq!(res, U128::ZERO);
        assert_eq!(carry, Limb::ONE);
    }

    #[test]
    fn wrapping_add_no_carry() {
        assert_eq!(U128::ZERO.wrapping_add(&U128::ONE), U128::ONE);
    }

    #[test]
    fn wrapping_add_with_carry() {
        assert_eq!(U128::MAX.wrapping_add(&U128::ONE), U128::ZERO);
    }


}

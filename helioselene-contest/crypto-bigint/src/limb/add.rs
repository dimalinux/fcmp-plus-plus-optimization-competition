//! Limb addition

use crate::{Limb, WideWord, Word};

impl Limb {
    /// Computes `self + rhs + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn adc(self, rhs: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = rhs.0 as WideWord;
        let carry = carry.0 as WideWord;
        let ret = a + b + carry;
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }
}

#[cfg(test)]
mod tests {
    use crate::Limb;

    #[test]
    fn adc_no_carry() {
        let (res, carry) = Limb::ZERO.adc(Limb::ONE, Limb::ZERO);
        assert_eq!(res, Limb::ONE);
        assert_eq!(carry, Limb::ZERO);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = Limb::MAX.adc(Limb::ONE, Limb::ZERO);
        assert_eq!(res, Limb::ZERO);
        assert_eq!(carry, Limb::ONE);
    }
}

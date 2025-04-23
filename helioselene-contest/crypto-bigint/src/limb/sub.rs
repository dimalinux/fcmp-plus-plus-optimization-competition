//! Limb subtraction

use crate::{Limb, WideWord, Word};

impl Limb {
    /// Computes `self - (rhs + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub const fn sbb(self, rhs: Limb, borrow: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = rhs.0 as WideWord;
        let borrow = (borrow.0 >> (Self::BITS - 1)) as WideWord;
        let ret = a.wrapping_sub(b + borrow);
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }
}

#[cfg(test)]
mod tests {
    use crate::Limb;

    #[test]
    fn sbb_no_borrow() {
        let (res, borrow) = Limb::ONE.sbb(Limb::ONE, Limb::ZERO);
        assert_eq!(res, Limb::ZERO);
        assert_eq!(borrow, Limb::ZERO);
    }

    #[test]
    fn sbb_with_borrow() {
        let (res, borrow) = Limb::ZERO.sbb(Limb::ONE, Limb::ZERO);

        assert_eq!(res, Limb::MAX);
        assert_eq!(borrow, Limb::MAX);
    }
}

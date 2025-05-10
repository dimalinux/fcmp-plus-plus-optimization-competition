//! Big integers are represented as an array of smaller CPU word-size integers
//! called "limbs".

use crate::bigint::ct_choice::CtChoice;

#[inline(always)]
pub(crate) const fn adc(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
    let a = lhs as u128;
    let b = rhs as u128;
    let carry = carry as u128;
    let ret = a + b + carry;
    (ret as u64, (ret >> u64::BITS) as u64)
}

/// Computes `self - (rhs + borrow)`, returning the result along with the new borrow.
#[inline(always)]
pub(crate) const fn sbb(lhs: u64, rhs: u64, borrow: u64) -> (u64, u64) {
    let a = lhs as u128;
    let b = rhs as u128;
    let borrow = (borrow >> (u64::BITS - 1)) as u128;
    let ret = a.wrapping_sub(b + borrow);
    (ret as u64, (ret >> u64::BITS) as u64)
}

/// Computes `self + (b * c) + carry`, returning the result along with the new carry.
#[inline(always)]
pub(crate) const fn mac(lhs: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let a = lhs as u128;
    let b = b as u128;
    let c = c as u128;
    let carry = carry as u128;
    let ret = a + (b * c) + carry;
    (ret as u64, (ret >> u64::BITS) as u64)
}

/// Return `b` if `c` is truthy, otherwise return `a`.
#[inline]
pub(crate) const fn ct_select(a: u64, b: u64, c: CtChoice) -> u64 {
    c.select(a, b)
}

/// Returns the truthy value if `self != 0` and the falsy value otherwise.
#[inline]
pub(crate) const fn ct_is_nonzero(w: u64) -> CtChoice {
    CtChoice::from_lsb((w | w.wrapping_neg()) >> (u64::BITS - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adc_no_carry() {
        let (res, carry) = adc(0, 1, 0);
        assert_eq!(res, 1);
        assert_eq!(carry, 0);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = adc(u64::MAX, 1, 0);
        assert_eq!(res, 0);
        assert_eq!(carry, 1);
    }

    #[test]
    fn sbb_no_borrow() {
        let (res, borrow) = sbb(1, 1, 0);
        assert_eq!(res, 0);
        assert_eq!(borrow, 0);
    }

    #[test]
    fn sbb_with_borrow() {
        let (res, borrow) = sbb(0, 1, 0);
        assert_eq!(res, u64::MAX);
        assert_eq!(borrow, u64::MAX);
    }
}

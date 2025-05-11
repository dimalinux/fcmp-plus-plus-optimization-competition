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
    // (x | x.wrapping_neg()) is 0 if and only if x == 0, otherwise
    // the MSB is set. We use sign-extension to convert the MSB value
    // into truthy or falsy.
    let mask = ((w | w.wrapping_neg()) as i64 >> 63) as u64;
    CtChoice::from_mask(mask)
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

    #[test]
    fn test_ct_is_nonzero() {
        assert!(!ct_is_nonzero(0).is_true_vartime());
        assert!(ct_is_nonzero(1).is_true_vartime());
        assert!(ct_is_nonzero(2).is_true_vartime());
        assert!(ct_is_nonzero(i64::MIN as u64).is_true_vartime());
        assert!(ct_is_nonzero(u64::MAX).is_true_vartime());
    }
}

//! Big integers are represented as an array of smaller CPU word-size integers
//! called "limbs".

use crate::u256::ct_choice::CtChoice;

pub(crate) const WORD_BITS: usize = u64::BITS as usize; // TODO: remove?
pub(crate) const WORD_BYTES: usize = WORD_BITS / 8;

/// Computes `lhs + rhs + carry`, returning the result along with the new carry
/// (0, 1, or 2).
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn carrying_add(lhs: u64, rhs: u64, carry: u64) -> (u64, u64) {
    let a = lhs as u128;
    let b = rhs as u128;
    let carry = carry as u128;
    let ret = a + b + carry;

    (ret as u64, (ret >> u64::BITS) as u64)
}

/// Computes `self - (rhs + borrow)`, returning the result along with the new borrow.
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn borrowing_sub(lhs: u64, rhs: u64, borrow: u64) -> (u64, u64) {
    let a = lhs as u128;
    let b = rhs as u128;
    let borrow = (borrow >> (u64::BITS - 1)) as u128;
    let ret = a.wrapping_sub(b + borrow);

    (ret as u64, (ret >> u64::BITS) as u64)
}

/// Computes `(lhs * rhs) + addend + carry`, returning the result along with the new carry.
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn carrying_mul_add(lhs: u64, rhs: u64, addend: u64, carry: u64) -> (u64, u64) {
    let lhs = lhs as u128;
    let rhs = rhs as u128;
    let addend = addend as u128;
    let carry = carry as u128;

    let ret = (lhs * rhs) + addend + carry;

    // (lo, hi)
    (ret as u64, (ret >> u64::BITS) as u64)
}

/// Return `b` if `c` is truthy, otherwise return `a`.
#[inline]
pub(crate) const fn ct_select(a: u64, b: u64, c: CtChoice) -> u64 {
    c.select(a, b)
}

/// Returns the truthy value if `self != 0` and the falsy value otherwise.
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn ct_is_nonzero(w: u64) -> CtChoice {
    // (x | x.wrapping_neg()) is 0 if and only if x == 0, otherwise
    // the MSB is set. We use sign-extension to convert the MSB value
    // into truthy or falsy.
    let mask = ((w | w.wrapping_neg()) as i64 >> 63) as u64;
    CtChoice::from_mask(mask)
}

#[inline]
#[allow(clippy::cast_possible_wrap)]
pub(crate) const fn ct_is_zero(w: u64) -> CtChoice {
    // m is 1 if any bit is set, otherwise zero.
    let m = (w | w.wrapping_neg()) >> 63;

    // convert zero to truthy and 1 to falsy.
    CtChoice::from_mask(m.wrapping_sub(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adc_no_carry() {
        let (res, carry) = carrying_add(0, 1, 0);
        assert_eq!(res, 1);
        assert_eq!(carry, 0);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = carrying_add(u64::MAX, 1, 0);
        assert_eq!(res, 0);
        assert_eq!(carry, 1);
    }

    #[test]
    fn sbb_no_borrow() {
        let (res, borrow) = borrowing_sub(1, 1, 0);
        assert_eq!(res, 0);
        assert_eq!(borrow, 0);
    }

    #[test]
    fn sbb_with_borrow() {
        let (res, borrow) = borrowing_sub(0, 1, 0);
        assert_eq!(res, u64::MAX);
        assert_eq!(borrow, u64::MAX);
    }

    #[test]
    #[allow(clippy::cast_sign_loss)]
    fn test_ct_is_nonzero() {
        assert!(!ct_is_nonzero(0).is_true_vartime());
        assert!(ct_is_nonzero(1).is_true_vartime());
        assert!(ct_is_nonzero(2).is_true_vartime());
        assert!(ct_is_nonzero(i64::MIN as u64).is_true_vartime());
        assert!(ct_is_nonzero(u64::MAX).is_true_vartime());
    }

    #[test]
    #[allow(clippy::cast_sign_loss)]
    fn test_ct_is_zero() {
        assert!(ct_is_zero(0).is_true_vartime());
        assert!(!ct_is_zero(1).is_true_vartime());
        assert!(!ct_is_zero(2).is_true_vartime());
        assert!(!ct_is_zero(i64::MIN as u64).is_true_vartime());
        assert!(!ct_is_zero(u64::MAX).is_true_vartime());
    }
}

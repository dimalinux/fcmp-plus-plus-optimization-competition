//! Big integers are represented as an array of smaller CPU word-size integers
//! called "limbs".

use subtle::Choice;

use crate::bigint::ct_choice::CtChoice;

/// Unsigned integer type that the [`Limb`] newtype wraps.
pub(crate) type Word = u64;

/// Wide integer type: double the width of [`Word`].
pub(crate) type WideWord = u128;

#[inline(always)]
pub(crate) const fn adc(lhs: Word, rhs: Word, carry: Word) -> (Word, Word) {
    let a = lhs as WideWord;
    let b = rhs as WideWord;
    let carry = carry as WideWord;
    let ret = a + b + carry;
    (ret as Word, (ret >> Word::BITS) as Word)
}

/// Computes `self - (rhs + borrow)`, returning the result along with the new borrow.
#[inline(always)]
pub(crate) const fn sbb(lhs: Word, rhs: Word, borrow: Word) -> (Word, Word) {
    let a = lhs as WideWord;
    let b = rhs as WideWord;
    let borrow = (borrow >> (Word::BITS - 1)) as WideWord;
    let ret = a.wrapping_sub(b + borrow);
    (ret as Word, (ret >> Word::BITS) as Word)
}

/// Computes `self + (b * c) + carry`, returning the result along with the new carry.
#[inline(always)]
pub(crate) const fn mac(lhs: Word, b: Word, c: Word, carry: Word) -> (Word, Word) {
    let a = lhs as WideWord;
    let b = b as WideWord;
    let c = c as WideWord;
    let carry = carry as WideWord;
    let ret = a + (b * c) + carry;
    (ret as Word, (ret >> Word::BITS) as Word)
}

/// Is this limb an odd number?
#[inline]
pub(crate) fn is_odd(w: Word) -> Choice {
    Choice::from(w as u8 & 1)
}

/// Return `b` if `c` is truthy, otherwise return `a`.
#[inline]
pub(crate) const fn ct_select(a: Word, b: Word, c: CtChoice) -> Word {
    c.select(a, b)
}

/// Returns the truthy value if `self != 0` and the falsy value otherwise.
#[inline]
pub(crate) const fn ct_is_nonzero(w: Word) -> CtChoice {
    CtChoice::from_lsb((w | w.wrapping_neg()) >> (Word::BITS - 1))
}

/// Returns the truthy value if `lhs == rhs` and the falsy value otherwise.
#[inline]
pub(crate) const fn ct_eq(x: Word, y: Word) -> CtChoice {
    // x ^ y == 0 if and only if x == y
    ct_is_nonzero(x ^ y).not()
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
        let (res, carry) = adc(Word::MAX, 1, 0);
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
        assert_eq!(res, Word::MAX);
        assert_eq!(borrow, Word::MAX);
    }
}

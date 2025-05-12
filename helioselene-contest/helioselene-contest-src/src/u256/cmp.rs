//! [`Uint`] comparisons.
//!
//! By default these are all constant-time and use the `subtle` crate.

use subtle::{Choice, ConstantTimeEq};

use crate::u256::{ct_choice::CtChoice, word, U256};

impl U256 {
    /// Return `b` if `c` is truthy, otherwise return `a`.
    #[inline]
    pub(crate) const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        let mut limbs = [0; Self::LIMBS];

        let mut i = 0;
        while i < Self::LIMBS {
            limbs[i] = word::ct_select(a.limbs[i], b.limbs[i], c);
            i += 1;
        }

        Self { limbs }
    }

    #[inline]
    pub(crate) const fn ct_swap(a: &Self, b: &Self, c: CtChoice) -> (Self, Self) {
        let new_a = Self::ct_select(a, b, c);
        let new_b = Self::ct_select(b, a, c);

        (new_a, new_b)
    }

    /// Returns the truthy value if `self`!=0 or the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_is_nonzero(&self) -> CtChoice {
        let w = self.limbs[0] | self.limbs[1] | self.limbs[2] | self.limbs[3];
        word::ct_is_nonzero(w)
    }

    /// Returns the truthy value if `self` is odd or the falsy value otherwise.
    pub(crate) const fn ct_is_odd(&self) -> CtChoice {
        CtChoice::from_lsb(self.limbs[0] & 1)
    }

    /// Returns the truthy value if `self == rhs` or the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_eq(lhs: &U256, rhs: &U256) -> CtChoice {
        let mut acc = lhs.limbs[0] ^ rhs.limbs[0];
        acc |= lhs.limbs[1] ^ rhs.limbs[1];
        acc |= lhs.limbs[2] ^ rhs.limbs[2];
        acc |= lhs.limbs[3] ^ rhs.limbs[3];

        // acc == 0 if and only if self == rhs
        word::ct_is_nonzero(acc).not()
    }

    /// Returns the truthy value if `self <= rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_lt(lhs: &Self, rhs: &Self) -> CtChoice {
        // We could use the same approach as in Limb::ct_lt(),
        // but since we have to use Uint::wrapping_sub(), which calls `sbb()`,
        // there are no savings compared to just calling `sbb()` directly.
        let (_res, borrow) = lhs.subtract_with_borrow(rhs, 0);
        CtChoice::from_mask(borrow)
    }
}

impl ConstantTimeEq for U256 {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        Self::ct_eq(self, other).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ct_eq_simple() {
        assert!(U256::ct_eq(&U256::ZERO, &U256::ZERO).is_true_vartime());
        assert!(U256::ct_eq(&U256::ONE, &U256::ONE).is_true_vartime());
        assert!(U256::ct_eq(&U256::MAX, &U256::MAX).is_true_vartime());
        assert!(!U256::ct_eq(&U256::ONE, &U256::ZERO).is_true_vartime());
        assert!(!U256::ct_eq(&U256::MAX, &U256::ONE).is_true_vartime());
    }

    #[test]
    fn ct_eq_high_bit_in_last_limb() {
        let a = U256::ZERO;
        let mut b = U256::ZERO;
        b.limbs[3] = i64::MIN as u64;

        assert!(!U256::ct_eq(&a, &b).is_true_vartime());
        assert!(U256::ct_eq(&b, &b).is_true_vartime());
    }
}

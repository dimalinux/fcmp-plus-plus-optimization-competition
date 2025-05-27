//! [`Uint`] comparisons.
//!
//! By default these are all constant-time.

use crate::u256::{ct_choice::CtChoice, primitives, U256};

impl U256 {
    /// Return `b` if `c` is truthy, otherwise return `a`.
    #[inline]
    pub(crate) const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        Self {
            limbs: [
                c.select(a.limbs[0], b.limbs[0]),
                c.select(a.limbs[1], b.limbs[1]),
                c.select(a.limbs[2], b.limbs[2]),
                c.select(a.limbs[3], b.limbs[3]),
            ],
        }
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
        primitives::ct_is_nonzero(w)
    }

    /// Returns the truthy value if `self` is odd or the falsy value otherwise.
    pub(crate) const fn ct_is_odd(&self) -> CtChoice {
        CtChoice::from_lsb(self.limbs[0] & 1)
    }

    /// Returns the truthy value if `self == rhs` or the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_eq(lhs: &Self, rhs: &Self) -> CtChoice {
        let mut acc = lhs.limbs[0] ^ rhs.limbs[0];
        acc |= lhs.limbs[1] ^ rhs.limbs[1];
        acc |= lhs.limbs[2] ^ rhs.limbs[2];
        acc |= lhs.limbs[3] ^ rhs.limbs[3];

        // acc == 0 if and only if self == rhs
        primitives::ct_is_nonzero(acc).not()
    }

    pub(crate) const fn ct_is_zero(&self) -> CtChoice {
        let v = self.limbs[0] | self.limbs[1] | self.limbs[2] | self.limbs[3];
        primitives::ct_is_zero(v)
    }

    /// Returns the truthy value if `self <= rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_lt(lhs: &Self, rhs: &Self) -> CtChoice {
        // We could use the same approach as in Limb::ct_lt(),
        // but since we have to use Uint::wrapping_sub(), which calls `sbb()`,
        // there are no savings compared to just calling `sbb()` directly.
        let (_res, borrow) = lhs.borrowing_sub(rhs, 0);
        CtChoice::from_mask(borrow)
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
    #[allow(clippy::cast_sign_loss)]
    fn ct_eq_high_bit_in_last_limb() {
        let a = U256::ZERO;
        let mut b = U256::ZERO;
        b.limbs[3] = i64::MIN as u64;

        assert!(!U256::ct_eq(&a, &b).is_true_vartime());
        assert!(U256::ct_eq(&b, &b).is_true_vartime());
    }
}

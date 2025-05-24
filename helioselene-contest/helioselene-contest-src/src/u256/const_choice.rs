use subtle::Choice;

/// A boolean value returned by constant-time `const fn`s.
// TODO: should be replaced by `subtle::Choice` or `CtOption`
// when `subtle` starts supporting const fns.
#[derive(Debug, Copy, Clone)]
pub(crate) struct ConstChoice(u64);

impl ConstChoice {
    /// The falsy value.
    pub(crate) const FALSY: Self = Self(0);
    /// The truthy value.
    pub(crate) const TRUTHY: Self = Self(u64::MAX);

    /// Returns the truthy value if `value == Word::MAX`, and the falsy value if `value == 0`.
    /// Panics for other values.
    #[inline(always)]
    pub(crate) const fn from_mask(value: u64) -> Self {
        debug_assert!(value == Self::FALSY.0 || value == Self::TRUTHY.0);
        Self(value)
    }

    /// Returns the truthy value if `value == 1`, and the falsy value if `value == 0`.
    /// Panics if any bit other than the least significant bit is set.
    #[inline(always)]
    pub(crate) const fn from_lsb(value: u64) -> Self {
        debug_assert!(value == 0 || value == 1);
        // Turns 0->0 (falsy), 1->u64::MAX (truthy)
        Self(value.wrapping_neg())
    }

    #[inline(always)]
    pub(crate) const fn not(self) -> Self {
        Self(!self.0)
    }

    /// Return `b` if `self` is truthy, otherwise return `a`.
    #[inline(always)]
    pub(crate) const fn select(self, a: u64, b: u64) -> u64 {
        a ^ (self.0 & (a ^ b))
    }

    /// Returns the truthy value if `x == y`, and the falsy value otherwise.
    #[inline]
    pub(crate) const fn from_u64_eq(x: u64, y: u64) -> Self {
        let diff = x ^ y;
        // (diff - 1) >> 63 yields 1 only when diff == 0
        let eq_mask = (diff.wrapping_sub(1) >> (u64::BITS - 1)).wrapping_neg();
        Self(eq_mask)
    }

    /// Return `x` if `self` is truthy, otherwise return 0.
    #[inline(always)]
    pub(crate) const fn if_true(self, x: u64) -> u64 {
        x & self.0
    }

    pub(crate) const fn is_true_vartime(self) -> bool {
        self.0 == Self::TRUTHY.0
    }

    #[inline(always)]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn to_u8(self) -> u8 {
        (self.0 as u8) & 1
    }
}

impl From<ConstChoice> for Choice {
    fn from(choice: ConstChoice) -> Self {
        Self::from(choice.to_u8())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select() {
        let a: u64 = 1;
        let b: u64 = 2;
        assert_eq!(ConstChoice::TRUTHY.select(a, b), b);
        assert_eq!(ConstChoice::FALSY.select(a, b), a);
    }

    #[test]
    #[allow(clippy::cast_sign_loss)]
    fn from_u64_eq() {
        assert_eq!(ConstChoice::from_u64_eq(0, 0).0, ConstChoice::TRUTHY.0);
        assert_eq!(ConstChoice::from_u64_eq(1, 2).0, ConstChoice::FALSY.0);

        let v = i64::MIN as u64; // wrapping_neg of this value equal itself
        assert_eq!(ConstChoice::from_u64_eq(v, v).0, ConstChoice::TRUTHY.0);
        assert_eq!(ConstChoice::from_u64_eq(0, v).0, ConstChoice::FALSY.0);
    }
}

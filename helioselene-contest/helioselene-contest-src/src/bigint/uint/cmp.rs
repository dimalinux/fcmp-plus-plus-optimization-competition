//! [`Uint`] comparisons.
//!
//! By default these are all constant-time and use the `subtle` crate.

use core::cmp::Ordering;

use subtle::{Choice, ConstantTimeEq, ConstantTimeGreater, ConstantTimeLess};

use crate::bigint::{ct_choice::CtChoice, uint::Limb, U256};

impl U256 {
    /// Return `b` if `c` is truthy, otherwise return `a`.
    #[inline]
    pub(crate) const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        let mut limbs = [Limb::ZERO; Self::LIMBS];

        let mut i = 0;
        while i < Self::LIMBS {
            limbs[i] = Limb::ct_select(a.limbs[i], b.limbs[i], c);
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
        let mut b = 0;
        let mut i = 0;
        while i < Self::LIMBS {
            b |= self.limbs[i].0;
            i += 1;
        }
        Limb(b).ct_is_nonzero()
    }

    /// Returns the truthy value if `self` is odd or the falsy value otherwise.
    pub(crate) const fn ct_is_odd(&self) -> CtChoice {
        CtChoice::from_lsb(self.limbs[0].0 & 1)
    }

    /// Returns the truthy value if `self == rhs` or the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_eq(lhs: &Self, rhs: &Self) -> CtChoice {
        let mut acc = 0;
        let mut i = 0;

        while i < Self::LIMBS {
            acc |= lhs.limbs[i].0 ^ rhs.limbs[i].0;
            i += 1;
        }

        // acc == 0 if and only if self == rhs
        Limb(acc).ct_is_nonzero().not()
    }

    /// Returns the truthy value if `self <= rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_lt(lhs: &Self, rhs: &Self) -> CtChoice {
        // We could use the same approach as in Limb::ct_lt(),
        // but since we have to use Uint::wrapping_sub(), which calls `sbb()`,
        // there are no savings compared to just calling `sbb()` directly.
        let (_res, borrow) = lhs.sbb(rhs, Limb::ZERO);
        CtChoice::from_mask(borrow.0)
    }

    /// Returns the truthy value if `self >= rhs` and the falsy value otherwise.
    #[inline]
    pub(crate) const fn ct_gt(lhs: &Self, rhs: &Self) -> CtChoice {
        let (_res, borrow) = rhs.sbb(lhs, Limb::ZERO);
        CtChoice::from_mask(borrow.0)
    }

    /// Returns the ordering between `self` and `rhs` as an i8.
    /// Values correspond to the Ordering enum:
    ///   -1 is Less
    ///   0 is Equal
    ///   1 is Greater
    #[inline]
    pub(crate) const fn ct_cmp(lhs: &Self, rhs: &Self) -> i8 {
        let mut i = 0;
        let mut borrow = Limb::ZERO;
        let mut diff = Limb::ZERO;

        while i < Self::LIMBS {
            let (w, b) = rhs.limbs[i].sbb(lhs.limbs[i], borrow);
            diff = diff.bitor(w);
            borrow = b;
            i += 1;
        }
        let sgn = ((borrow.0 & 2) as i8) - 1;
        (diff.ct_is_nonzero().to_u8() as i8) * sgn
    }
}

impl ConstantTimeEq for U256 {
    #[inline]
    fn ct_eq(&self, other: &Self) -> Choice {
        Self::ct_eq(self, other).into()
    }
}

impl ConstantTimeGreater for U256 {
    #[inline]
    fn ct_gt(&self, other: &Self) -> Choice {
        Self::ct_gt(self, other).into()
    }
}

impl ConstantTimeLess for U256 {
    #[inline]
    fn ct_lt(&self, other: &Self) -> Choice {
        Self::ct_lt(self, other).into()
    }
}

impl Eq for U256 {}

impl Ord for U256 {
    fn cmp(&self, other: &Self) -> Ordering {
        let c = Self::ct_cmp(self, other);
        match c {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for U256 {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other).into()
    }
}

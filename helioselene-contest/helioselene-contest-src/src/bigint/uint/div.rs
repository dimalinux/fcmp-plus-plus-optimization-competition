//! [`Uint`] division operations.

use crate::bigint::{
    ct_choice::CtChoice,
    non_zero::NonZero,
    uint::{Limb, Uint, Word},
};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `self` / `rhs`, returns the quotient (q), remainder (r)
    /// and the truthy value for is_some or the falsy value for is_none.
    ///
    /// NOTE: Use only if you need to access const fn. Otherwise use [`Self::div_rem`] because
    /// the value for is_some needs to be checked before using `q` and `r`.
    ///
    /// This is variable only with respect to `rhs`.
    ///
    /// When used with a fixed `rhs`, this function is constant-time with respect
    /// to `self`.
    pub(crate) const fn ct_div_rem(&self, rhs: &Self) -> (Self, Self, CtChoice) {
        let mb = rhs.bits_vartime();
        let mut bd = Self::BITS - mb;
        let mut rem = *self;
        let mut quo = Self::ZERO;
        let mut c = rhs.shl_vartime(bd);

        loop {
            let (mut r, borrow) = rem.sbb(&c, Limb::ZERO);
            rem = Self::ct_select(&r, &rem, CtChoice::from_mask(borrow.0));
            r = quo.bitor(&Self::ONE);
            quo = Self::ct_select(&r, &quo, CtChoice::from_mask(borrow.0));
            if bd == 0 {
                break;
            }
            bd -= 1;
            c = c.shr_vartime(1);
            quo = quo.shl_vartime(1);
        }

        let is_some = Limb(mb as Word).ct_is_nonzero();
        quo = Self::ct_select(&Self::ZERO, &quo, is_some);
        (quo, rem, is_some)
    }

    /// Computes `self` % `rhs`, returns the remainder and
    /// and the truthy value for is_some or the falsy value for is_none.
    ///
    /// NOTE: Use only if you need to access const fn. Otherwise use [`Self::rem`].
    /// This is variable only with respect to `rhs`.
    ///
    /// When used with a fixed `rhs`, this function is constant-time with respect
    /// to `self`.
    pub(crate) const fn const_rem(&self, rhs: &Self) -> (Self, CtChoice) {
        let mb = rhs.bits_vartime();
        let mut bd = Self::BITS - mb;
        let mut rem = *self;
        let mut c = rhs.shl_vartime(bd);

        loop {
            let (r, borrow) = rem.sbb(&c, Limb::ZERO);
            rem = Self::ct_select(&r, &rem, CtChoice::from_mask(borrow.0));
            if bd == 0 {
                break;
            }
            bd -= 1;
            c = c.shr_vartime(1);
        }

        let is_some = Limb(mb as Word).ct_is_nonzero();
        (rem, is_some)
    }

    /// Computes `self` % `rhs`, returns the remainder and
    /// and the truthy value for is_some or the falsy value for is_none.
    ///
    /// This is variable only with respect to `rhs`.
    ///
    /// When used with a fixed `rhs`, this function is constant-time with respect
    /// to `self`.
    pub(crate) const fn const_rem_wide(lower_upper: (Self, Self), rhs: &Self) -> (Self, CtChoice) {
        let mb = rhs.bits_vartime();

        // The number of bits to consider is two sets of limbs * BITS - mb (modulus bitcount)
        let mut bd = (2 * Self::BITS) - mb;

        // The wide integer to reduce, split into two halves
        let (mut lower, mut upper) = lower_upper;

        // Factor of the modulus, split into two halves
        let mut c = Self::shl_vartime_wide((*rhs, Uint::ZERO), bd);

        loop {
            let (lower_sub, borrow) = lower.sbb(&c.0, Limb::ZERO);
            let (upper_sub, borrow) = upper.sbb(&c.1, borrow);

            lower = Self::ct_select(&lower_sub, &lower, CtChoice::from_mask(borrow.0));
            upper = Self::ct_select(&upper_sub, &upper, CtChoice::from_mask(borrow.0));
            if bd == 0 {
                break;
            }
            bd -= 1;
            c = Self::shr_vartime_wide(c, 1);
        }

        let is_some = Limb(mb as Word).ct_is_nonzero();
        (lower, is_some)
    }

    /// Computes self % rhs, returns the remainder.
    pub(crate) fn rem(&self, rhs: &NonZero<Self>) -> Self {
        // Since `rhs` is nonzero, this should always hold.
        let (r, _c) = self.const_rem(rhs);
        r
    }

    /// Wrapped division is just normal division i.e. `self` / `rhs`
    /// There’s no way wrapping could ever happen.
    /// This function exists, so that all operations are accounted for in the wrapping operations.
    ///
    /// Panics if `rhs == 0`.
    pub const fn wrapping_div(&self, rhs: &Self) -> Self {
        let (q, _, c) = self.ct_div_rem(rhs);
        assert!(c.is_true_vartime(), "divide by zero");
        q
    }
}

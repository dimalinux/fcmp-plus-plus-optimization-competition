//! [`Uint`] division operations.

use crate::bigint::{ct_choice::CtChoice, word, U256};

impl U256 {
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
            let (mut r, borrow) = rem.subtract_with_borrow(&c, 0);
            rem = Self::ct_select(&r, &rem, CtChoice::from_mask(borrow));
            r = quo.bitor(&Self::ONE);
            quo = Self::ct_select(&r, &quo, CtChoice::from_mask(borrow));
            if bd == 0 {
                break;
            }
            bd -= 1;
            c = c.shr_vartime(1);
            quo = quo.shl_vartime(1);
        }

        let is_some = word::ct_is_nonzero(mb as u64);
        quo = Self::ct_select(&Self::ZERO, &quo, is_some);
        (quo, rem, is_some)
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

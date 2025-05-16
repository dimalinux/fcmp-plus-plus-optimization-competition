//! [`Uint`] addition operations.

use crate::u256::{primitives::carrying_mul_add, U256};

impl U256 {
    /// Compute "wide" multiplication, with a product twice the size of the input.
    ///
    /// Returns a tuple containing the `(lo, hi)` components of the product.
    pub(crate) const fn mul_wide(&self, rhs: &Self) -> (Self, Self) {
        let mut lo = Self::ZERO;
        let mut hi = Self::ZERO;
        let mut carry: u64;

        // Using schoolbook multiplication.

        // i = 0
        (lo.limbs[0], carry) = carrying_mul_add(rhs.limbs[0], self.limbs[0], lo.limbs[0], 0);
        (lo.limbs[1], carry) = carrying_mul_add(rhs.limbs[1], self.limbs[0], lo.limbs[1], carry);
        (lo.limbs[2], carry) = carrying_mul_add(rhs.limbs[2], self.limbs[0], lo.limbs[2], carry);
        (lo.limbs[3], hi.limbs[0]) =
            carrying_mul_add(rhs.limbs[3], self.limbs[0], lo.limbs[3], carry);

        // i = 1
        (lo.limbs[1], carry) = carrying_mul_add(rhs.limbs[0], self.limbs[1], lo.limbs[1], 0);
        (lo.limbs[2], carry) = carrying_mul_add(rhs.limbs[1], self.limbs[1], lo.limbs[2], carry);
        (lo.limbs[3], carry) = carrying_mul_add(rhs.limbs[2], self.limbs[1], lo.limbs[3], carry);
        (hi.limbs[0], hi.limbs[1]) =
            carrying_mul_add(rhs.limbs[3], self.limbs[1], hi.limbs[0], carry);

        // i = 2
        (lo.limbs[2], carry) = carrying_mul_add(rhs.limbs[0], self.limbs[2], lo.limbs[2], 0);
        (lo.limbs[3], carry) = carrying_mul_add(rhs.limbs[1], self.limbs[2], lo.limbs[3], carry);
        (hi.limbs[0], carry) = carrying_mul_add(rhs.limbs[2], self.limbs[2], hi.limbs[0], carry);
        (hi.limbs[1], hi.limbs[2]) =
            carrying_mul_add(rhs.limbs[3], self.limbs[2], hi.limbs[1], carry);

        // i = 3
        (lo.limbs[3], carry) = carrying_mul_add(rhs.limbs[0], self.limbs[3], lo.limbs[3], 0);
        (hi.limbs[0], carry) = carrying_mul_add(rhs.limbs[1], self.limbs[3], hi.limbs[0], carry);
        (hi.limbs[1], carry) = carrying_mul_add(rhs.limbs[2], self.limbs[3], hi.limbs[1], carry);
        (hi.limbs[2], hi.limbs[3]) =
            carrying_mul_add(rhs.limbs[3], self.limbs[3], hi.limbs[2], carry);

        (lo, hi)
    }

    /// Square self, returning a "wide" result in two parts as (lo, hi).
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn square_wide(&self) -> (Self, Self) {
        // Translated from https://github.com/ucbrise/jedi-pairing/blob/c4bf151/include/core/bigint.hpp#L410
        //
        // Permission to relicense the resulting translation as Apache 2.0 + MIT was given
        // by the original author Sam Kumar: https://github.com/RustCrypto/crypto-bigint/pull/133#discussion_r1056870411

        let mut lo = Self::ZERO;
        let mut hi = Self::ZERO;
        let mut carry: u64;

        // Schoolbook multiplication, but only considering half of the multiplication grid

        // i = 1, j = 0
        (lo.limbs[1], lo.limbs[2]) = carrying_mul_add(self.limbs[0], self.limbs[1], lo.limbs[1], 0);

        // i = 2, j = 0
        (lo.limbs[2], carry) = carrying_mul_add(self.limbs[0], self.limbs[2], lo.limbs[2], 0);

        // i = 2, j = 1
        (lo.limbs[3], hi.limbs[0]) =
            carrying_mul_add(self.limbs[1], self.limbs[2], lo.limbs[3], carry);

        // i = 3, j = 0
        (lo.limbs[3], carry) = carrying_mul_add(self.limbs[0], self.limbs[3], lo.limbs[3], 0);

        // i = 3, j = 1
        (hi.limbs[0], carry) = carrying_mul_add(self.limbs[1], self.limbs[3], hi.limbs[0], carry);

        // i = 3, j = 2
        (hi.limbs[1], hi.limbs[2]) =
            carrying_mul_add(self.limbs[2], self.limbs[3], hi.limbs[1], carry);

        // Double the current result, this accounts for the other half of the multiplication grid.
        // TODO: The top word is empty so we can also use a special purpose shl.
        (lo, hi) = Self::shl_vartime_wide((lo, hi), 1);

        // Handle the diagonal

        // i = 0
        (lo.limbs[0], carry) = carrying_mul_add(self.limbs[0], self.limbs[0], lo.limbs[0], 0);

        let n = lo.limbs[1] as u128 + carry as u128;
        lo.limbs[1] = n as u64;
        carry = (n >> 64) as u64;

        // i = 1
        let (n, mut carry) = carrying_mul_add(self.limbs[1], self.limbs[1], lo.limbs[2], carry);
        lo.limbs[2] = n;

        let n = lo.limbs[3] as u128 + carry as u128;
        lo.limbs[3] = n as u64;
        carry = (n >> 64) as u64;

        // i = 2
        (hi.limbs[0], carry) = carrying_mul_add(self.limbs[2], self.limbs[2], hi.limbs[0], carry);

        let n = hi.limbs[1] as u128 + carry as u128;
        hi.limbs[1] = n as u64;
        carry = (n >> 64) as u64;

        // i = 3
        (hi.limbs[2], carry) = carrying_mul_add(self.limbs[3], self.limbs[3], hi.limbs[2], carry);

        let n = hi.limbs[3] as u128 + carry as u128;
        hi.limbs[3] = n as u64;

        (lo, hi)
    }
}

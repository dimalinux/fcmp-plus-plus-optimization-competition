//! [`Uint`] addition operations.

use crate::bigint::{word, U256};

impl U256 {
    /// Compute "wide" multiplication, with a product twice the size of the input.
    ///
    /// Returns a tuple containing the `(lo, hi)` components of the product.
    pub const fn mul_wide(&self, rhs: &U256) -> (U256, U256) {
        let mut lo = U256::ZERO;
        let mut hi = U256::ZERO;
        let mut carry: u64;

        // Using schoolbook multiplication.

        // i = 0
        carry = 0;
        let (w, c) = word::mac(lo.limbs[0], self.limbs[0], rhs.limbs[0], carry);
        lo.limbs[0] = w;
        carry = c;
        let (w, c) = word::mac(lo.limbs[1], self.limbs[0], rhs.limbs[1], carry);
        lo.limbs[1] = w;
        carry = c;
        let (w, c) = word::mac(lo.limbs[2], self.limbs[0], rhs.limbs[2], carry);
        lo.limbs[2] = w;
        carry = c;
        let (w, c) = word::mac(lo.limbs[3], self.limbs[0], rhs.limbs[3], carry);
        lo.limbs[3] = w;
        carry = c;
        hi.limbs[0] = carry;

        // i = 1
        carry = 0;
        let (w, c) = word::mac(lo.limbs[1], self.limbs[1], rhs.limbs[0], carry);
        lo.limbs[1] = w;
        carry = c;
        let (w, c) = word::mac(lo.limbs[2], self.limbs[1], rhs.limbs[1], carry);
        lo.limbs[2] = w;
        carry = c;
        let (w, c) = word::mac(lo.limbs[3], self.limbs[1], rhs.limbs[2], carry);
        lo.limbs[3] = w;
        carry = c;
        let (w, c) = word::mac(hi.limbs[0], self.limbs[1], rhs.limbs[3], carry);
        hi.limbs[0] = w;
        carry = c;
        hi.limbs[1] = carry;

        // i = 2
        carry = 0;
        let (w, c) = word::mac(lo.limbs[2], self.limbs[2], rhs.limbs[0], carry);
        lo.limbs[2] = w;
        carry = c;
        let (w, c) = word::mac(lo.limbs[3], self.limbs[2], rhs.limbs[1], carry);
        lo.limbs[3] = w;
        carry = c;
        let (w, c) = word::mac(hi.limbs[0], self.limbs[2], rhs.limbs[2], carry);
        hi.limbs[0] = w;
        carry = c;
        let (w, c) = word::mac(hi.limbs[1], self.limbs[2], rhs.limbs[3], carry);
        hi.limbs[1] = w;
        carry = c;
        hi.limbs[2] = carry;

        // i = 3
        carry = 0;
        let (w, c) = word::mac(lo.limbs[3], self.limbs[3], rhs.limbs[0], carry);
        lo.limbs[3] = w;
        carry = c;
        let (w, c) = word::mac(hi.limbs[0], self.limbs[3], rhs.limbs[1], carry);
        hi.limbs[0] = w;
        carry = c;
        let (w, c) = word::mac(hi.limbs[1], self.limbs[3], rhs.limbs[2], carry);
        hi.limbs[1] = w;
        carry = c;
        let (w, c) = word::mac(hi.limbs[2], self.limbs[3], rhs.limbs[3], carry);
        hi.limbs[2] = w;
        carry = c;
        hi.limbs[3] = carry;

        (lo, hi)
    }

    /// Square self, returning a "wide" result in two parts as (lo, hi).
    pub const fn square_wide(&self) -> (U256, U256) {
        // Translated from https://github.com/ucbrise/jedi-pairing/blob/c4bf151/include/core/bigint.hpp#L410
        //
        // Permission to relicense the resulting translation as Apache 2.0 + MIT was given
        // by the original author Sam Kumar: https://github.com/RustCrypto/crypto-bigint/pull/133#discussion_r1056870411
        const LIMBS: usize = U256::LIMBS;
        let mut lo = U256::ZERO;
        let mut hi = U256::ZERO;

        // Schoolbook multiplication, but only considering half of the multiplication grid
        let mut i = 1;
        while i < LIMBS {
            let mut j = 0;
            let mut carry = 0;

            while j < i {
                let k = i + j;

                if k >= LIMBS {
                    let (n, c) =
                        word::mac(hi.limbs[k - LIMBS], self.limbs[i], self.limbs[j], carry);
                    hi.limbs[k - LIMBS] = n;
                    carry = c;
                } else {
                    let (n, c) = word::mac(lo.limbs[k], self.limbs[i], self.limbs[j], carry);
                    lo.limbs[k] = n;
                    carry = c;
                }

                j += 1;
            }

            if (2 * i) < LIMBS {
                lo.limbs[2 * i] = carry;
            } else {
                hi.limbs[2 * i - LIMBS] = carry;
            }

            i += 1;
        }

        // Double the current result, this accounts for the other half of the multiplication grid.
        // TODO: The top word is empty so we can also use a special purpose shl.
        (lo, hi) = Self::shl_vartime_wide((lo, hi), 1);

        // Handle the diagonal of the multiplication grid, which finishes the multiplication grid.
        let mut carry = 0;
        let mut i = 0;
        while i < LIMBS {
            if (i * 2) < LIMBS {
                let (n, c) = word::mac(lo.limbs[i * 2], self.limbs[i], self.limbs[i], carry);
                lo.limbs[i * 2] = n;
                carry = c;
            } else {
                let (n, c) =
                    word::mac(hi.limbs[i * 2 - LIMBS], self.limbs[i], self.limbs[i], carry);
                hi.limbs[i * 2 - LIMBS] = n;
                carry = c;
            }

            if (i * 2 + 1) < LIMBS {
                let n = lo.limbs[i * 2 + 1] as u128 + carry as u128;
                lo.limbs[i * 2 + 1] = n as u64;
                carry = (n >> u64::BITS) as u64;
            } else {
                let n = hi.limbs[i * 2 + 1 - LIMBS] as u128 + carry as u128;
                hi.limbs[i * 2 + 1 - LIMBS] = n as u64;
                carry = (n >> u64::BITS) as u64;
            }

            i += 1;
        }

        (lo, hi)
    }
}

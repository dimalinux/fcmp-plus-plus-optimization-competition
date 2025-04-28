//! [`Uint`] addition operations.

use core::ops::Mul;

use crate::bigint::{
    limb::{Limb, WideWord, Word},
    uint::Uint,
    Concat, ConcatMixed,
};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Multiply `self` by `rhs`, returning a concatenated "wide" result.
    pub fn mul<const HLIMBS: usize>(
        &self,
        rhs: &Uint<HLIMBS>,
    ) -> <Uint<HLIMBS> as ConcatMixed<Self>>::MixedOutput
    where
        Uint<HLIMBS>: ConcatMixed<Self>,
    {
        let (lo, hi) = self.mul_wide(rhs);
        hi.concat_mixed(&lo)
    }

    /// Compute "wide" multiplication, with a product twice the size of the input.
    ///
    /// Returns a tuple containing the `(lo, hi)` components of the product.
    pub const fn mul_wide<const HLIMBS: usize>(&self, rhs: &Uint<HLIMBS>) -> (Self, Uint<HLIMBS>) {
        let mut i = 0;
        let mut lo = Self::ZERO;
        let mut hi = Uint::<HLIMBS>::ZERO;

        // Schoolbook multiplication.
        // TODO(tarcieri): use Karatsuba for better performance?
        while i < LIMBS {
            let mut j = 0;
            let mut carry = Limb::ZERO;

            while j < HLIMBS {
                let k = i + j;

                if k >= LIMBS {
                    let (n, c) = hi.limbs[k - LIMBS].mac(self.limbs[i], rhs.limbs[j], carry);
                    hi.limbs[k - LIMBS] = n;
                    carry = c;
                } else {
                    let (n, c) = lo.limbs[k].mac(self.limbs[i], rhs.limbs[j], carry);
                    lo.limbs[k] = n;
                    carry = c;
                }

                j += 1;
            }

            if i + j >= LIMBS {
                hi.limbs[i + j - LIMBS] = carry;
            } else {
                lo.limbs[i + j] = carry;
            }
            i += 1;
        }

        (lo, hi)
    }

    /// Perform saturating multiplication, returning `MAX` on overflow.
    pub const fn saturating_mul<const HLIMBS: usize>(&self, rhs: &Uint<HLIMBS>) -> Self {
        let (res, overflow) = self.mul_wide(rhs);
        Self::ct_select(&res, &Self::MAX, overflow.ct_is_nonzero())
    }

    /// Perform wrapping multiplication, discarding overflow.
    pub const fn wrapping_mul<const H: usize>(&self, rhs: &Uint<H>) -> Self {
        self.mul_wide(rhs).0
    }

    /// Square self, returning a concatenated "wide" result.
    pub fn square(&self) -> <Self as Concat>::Output
    where
        Self: Concat,
    {
        let (lo, hi) = self.square_wide();
        hi.concat(&lo)
    }

    /// Square self, returning a "wide" result in two parts as (lo, hi).
    pub const fn square_wide(&self) -> (Self, Self) {
        // Translated from https://github.com/ucbrise/jedi-pairing/blob/c4bf151/include/core/bigint.hpp#L410
        //
        // Permission to relicense the resulting translation as Apache 2.0 + MIT was given
        // by the original author Sam Kumar: https://github.com/RustCrypto/crypto-bigint/pull/133#discussion_r1056870411
        let mut lo = Self::ZERO;
        let mut hi = Self::ZERO;

        // Schoolbook multiplication, but only considering half of the multiplication grid
        let mut i = 1;
        while i < LIMBS {
            let mut j = 0;
            let mut carry = Limb::ZERO;

            while j < i {
                let k = i + j;

                if k >= LIMBS {
                    let (n, c) = hi.limbs[k - LIMBS].mac(self.limbs[i], self.limbs[j], carry);
                    hi.limbs[k - LIMBS] = n;
                    carry = c;
                } else {
                    let (n, c) = lo.limbs[k].mac(self.limbs[i], self.limbs[j], carry);
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
        let mut carry = Limb::ZERO;
        let mut i = 0;
        while i < LIMBS {
            if (i * 2) < LIMBS {
                let (n, c) = lo.limbs[i * 2].mac(self.limbs[i], self.limbs[i], carry);
                lo.limbs[i * 2] = n;
                carry = c;
            } else {
                let (n, c) = hi.limbs[i * 2 - LIMBS].mac(self.limbs[i], self.limbs[i], carry);
                hi.limbs[i * 2 - LIMBS] = n;
                carry = c;
            }

            if (i * 2 + 1) < LIMBS {
                let n = lo.limbs[i * 2 + 1].0 as WideWord + carry.0 as WideWord;
                lo.limbs[i * 2 + 1] = Limb(n as Word);
                carry = Limb((n >> Word::BITS) as Word);
            } else {
                let n = hi.limbs[i * 2 + 1 - LIMBS].0 as WideWord + carry.0 as WideWord;
                hi.limbs[i * 2 + 1 - LIMBS] = Limb(n as Word);
                carry = Limb((n >> Word::BITS) as Word);
            }

            i += 1;
        }

        (lo, hi)
    }
}

impl<const LIMBS: usize, const HLIMBS: usize> Mul<Uint<HLIMBS>> for Uint<LIMBS>
where
    Uint<HLIMBS>: ConcatMixed<Uint<LIMBS>>,
{
    type Output = <Uint<HLIMBS> as ConcatMixed<Self>>::MixedOutput;

    fn mul(self, other: Uint<HLIMBS>) -> Self::Output {
        Uint::mul(&self, &other)
    }
}

impl<const LIMBS: usize, const HLIMBS: usize> Mul<&Uint<HLIMBS>> for Uint<LIMBS>
where
    Uint<HLIMBS>: ConcatMixed<Uint<LIMBS>>,
{
    type Output = <Uint<HLIMBS> as ConcatMixed<Self>>::MixedOutput;

    fn mul(self, other: &Uint<HLIMBS>) -> Self::Output {
        Uint::mul(&self, other)
    }
}

impl<const LIMBS: usize, const HLIMBS: usize> Mul<Uint<HLIMBS>> for &Uint<LIMBS>
where
    Uint<HLIMBS>: ConcatMixed<Uint<LIMBS>>,
{
    type Output = <Uint<HLIMBS> as ConcatMixed<Uint<LIMBS>>>::MixedOutput;

    fn mul(self, other: Uint<HLIMBS>) -> Self::Output {
        Uint::mul(self, &other)
    }
}

impl<const LIMBS: usize, const HLIMBS: usize> Mul<&Uint<HLIMBS>> for &Uint<LIMBS>
where
    Uint<HLIMBS>: ConcatMixed<Uint<LIMBS>>,
{
    type Output = <Uint<HLIMBS> as ConcatMixed<Uint<LIMBS>>>::MixedOutput;

    fn mul(self, other: &Uint<HLIMBS>) -> Self::Output {
        Uint::mul(self, other)
    }
}

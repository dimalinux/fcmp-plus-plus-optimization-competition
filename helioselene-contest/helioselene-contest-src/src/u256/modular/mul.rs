//! Multiplications between integers in Montgomery form with a constant modulus.

use core::marker::PhantomData;

use crate::u256::{primitives::carrying_mul_add, MontyForm, MontyParams, U256};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Multiplies by `rhs`.
    pub(crate) const fn mul(&self, rhs: &Self) -> Self {
        let product = Self::mul_wide(&self.montgomery_form, &rhs.montgomery_form);
        Self {
            montgomery_form: Self::montgomery_reduction(&product),
            phantom: PhantomData,
        }
    }

    /// Computes the (reduced) square.
    pub(crate) const fn square(&self) -> Self {
        let product = Self::square_wide(&self.montgomery_form);
        let montgomery_form = Self::montgomery_reduction(&product);
        Self {
            montgomery_form,
            phantom: PhantomData,
        }
    }

    /// Compute "wide" multiplication, with a product twice the size of the input.
    ///
    /// Returns a tuple containing the `(lo, hi)` components of the product.
    pub(super) const fn mul_wide(lhs: &U256, rhs: &U256) -> (U256, U256) {
        let mut lo = U256::ZERO;
        let mut hi = U256::ZERO;
        let mut carry: u64;

        // Using schoolbook multiplication.

        // i = 0
        (lo.limbs[0], carry) = carrying_mul_add(rhs.limbs[0], lhs.limbs[0], lo.limbs[0], 0);
        (lo.limbs[1], carry) = carrying_mul_add(rhs.limbs[1], lhs.limbs[0], lo.limbs[1], carry);
        (lo.limbs[2], carry) = carrying_mul_add(rhs.limbs[2], lhs.limbs[0], lo.limbs[2], carry);
        (lo.limbs[3], hi.limbs[0]) =
            carrying_mul_add(rhs.limbs[3], lhs.limbs[0], lo.limbs[3], carry);

        // i = 1
        (lo.limbs[1], carry) = carrying_mul_add(rhs.limbs[0], lhs.limbs[1], lo.limbs[1], 0);
        (lo.limbs[2], carry) = carrying_mul_add(rhs.limbs[1], lhs.limbs[1], lo.limbs[2], carry);
        (lo.limbs[3], carry) = carrying_mul_add(rhs.limbs[2], lhs.limbs[1], lo.limbs[3], carry);
        (hi.limbs[0], hi.limbs[1]) =
            carrying_mul_add(rhs.limbs[3], lhs.limbs[1], hi.limbs[0], carry);

        // i = 2
        (lo.limbs[2], carry) = carrying_mul_add(rhs.limbs[0], lhs.limbs[2], lo.limbs[2], 0);
        (lo.limbs[3], carry) = carrying_mul_add(rhs.limbs[1], lhs.limbs[2], lo.limbs[3], carry);
        (hi.limbs[0], carry) = carrying_mul_add(rhs.limbs[2], lhs.limbs[2], hi.limbs[0], carry);
        (hi.limbs[1], hi.limbs[2]) =
            carrying_mul_add(rhs.limbs[3], lhs.limbs[2], hi.limbs[1], carry);

        // i = 3
        (lo.limbs[3], carry) = carrying_mul_add(rhs.limbs[0], lhs.limbs[3], lo.limbs[3], 0);
        (hi.limbs[0], carry) = carrying_mul_add(rhs.limbs[1], lhs.limbs[3], hi.limbs[0], carry);
        (hi.limbs[1], carry) = carrying_mul_add(rhs.limbs[2], lhs.limbs[3], hi.limbs[1], carry);
        (hi.limbs[2], hi.limbs[3]) =
            carrying_mul_add(rhs.limbs[3], lhs.limbs[3], hi.limbs[2], carry);

        (lo, hi)
    }

    /// Square self, returning a "wide" result in two parts as (lo, hi).
    #[allow(clippy::cast_possible_truncation)]
    const fn square_wide(num: &U256) -> (U256, U256) {
        let mut lo = U256::ZERO;
        let mut hi = U256::ZERO;
        let mut carry: u64;

        // Schoolbook multiplication, but only considering half of the multiplication grid

        // i = 1, j = 0
        (lo.limbs[1], lo.limbs[2]) = carrying_mul_add(num.limbs[0], num.limbs[1], lo.limbs[1], 0);

        // i = 2, j = 0
        (lo.limbs[2], carry) = carrying_mul_add(num.limbs[0], num.limbs[2], lo.limbs[2], 0);

        // i = 2, j = 1
        (lo.limbs[3], hi.limbs[0]) =
            carrying_mul_add(num.limbs[1], num.limbs[2], lo.limbs[3], carry);

        // i = 3, j = 0
        (lo.limbs[3], carry) = carrying_mul_add(num.limbs[0], num.limbs[3], lo.limbs[3], 0);

        // i = 3, j = 1
        (hi.limbs[0], carry) = carrying_mul_add(num.limbs[1], num.limbs[3], hi.limbs[0], carry);

        // i = 3, j = 2
        (hi.limbs[1], hi.limbs[2]) =
            carrying_mul_add(num.limbs[2], num.limbs[3], hi.limbs[1], carry);

        // Double the current result, this accounts for the other half of the multiplication grid.
        // TODO: The top word is empty so we can also use a special purpose shl.
        (lo, hi) = U256::shl_vartime_wide((lo, hi), 1);

        // Handle the diagonal

        // i = 0
        (lo.limbs[0], carry) = carrying_mul_add(num.limbs[0], num.limbs[0], lo.limbs[0], 0);

        let n = lo.limbs[1] as u128 + carry as u128;
        lo.limbs[1] = n as u64;
        carry = (n >> 64) as u64;

        // i = 1
        let (n, mut carry) = carrying_mul_add(num.limbs[1], num.limbs[1], lo.limbs[2], carry);
        lo.limbs[2] = n;

        let n = lo.limbs[3] as u128 + carry as u128;
        lo.limbs[3] = n as u64;
        carry = (n >> 64) as u64;

        // i = 2
        (hi.limbs[0], carry) = carrying_mul_add(num.limbs[2], num.limbs[2], hi.limbs[0], carry);

        let n = hi.limbs[1] as u128 + carry as u128;
        hi.limbs[1] = n as u64;
        carry = (n >> 64) as u64;

        // i = 3
        (hi.limbs[2], carry) = carrying_mul_add(num.limbs[3], num.limbs[3], hi.limbs[2], carry);

        let n = hi.limbs[3] as u128 + carry as u128;
        hi.limbs[3] = n as u64;

        (lo, hi)
    }
}

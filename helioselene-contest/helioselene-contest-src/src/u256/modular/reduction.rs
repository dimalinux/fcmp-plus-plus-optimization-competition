//! Modular reduction implementation.

use crate::u256::{
    primitives::{carrying_add, carrying_mul_add},
    MontyForm, MontyParams, U256,
};

impl<MOD: MontyParams> MontyForm<MOD> {
    /// Algorithm 14.32 in Handbook of Applied Cryptography <https://cacr.uwaterloo.ca/hac/about/chap14.pdf>
    pub(super) const fn montgomery_reduction(lower_upper: &(U256, U256)) -> U256 {
        let (mut lower, mut upper) = *lower_upper;
        let mut meta_carry = 0;

        // i = 0
        let u = lower.limbs[0].wrapping_mul(MOD::MOD_NEG_INV);
        let (_, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[0], lower.limbs[0], 0);

        // j = 1
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[1], lower.limbs[1], carry);
        lower.limbs[1] = new_limb;
        // j = 2
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[2], lower.limbs[2], carry);
        lower.limbs[2] = new_limb;
        // j = 3
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[3], lower.limbs[3], carry);
        lower.limbs[3] = new_limb;

        let (new_sum, new_meta_carry) = carrying_add(upper.limbs[0], carry, meta_carry);
        upper.limbs[0] = new_sum;
        meta_carry = new_meta_carry;

        // i = 1
        let u = lower.limbs[1].wrapping_mul(MOD::MOD_NEG_INV);
        let (_, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[0], lower.limbs[1], 0);

        // j = 1
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[1], lower.limbs[2], carry);
        lower.limbs[2] = new_limb;
        // j = 2
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[2], lower.limbs[3], carry);
        lower.limbs[3] = new_limb;
        // j = 3
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[3], upper.limbs[0], carry);
        upper.limbs[0] = new_limb;

        let (new_sum, new_meta_carry) = carrying_add(upper.limbs[1], carry, meta_carry);
        upper.limbs[1] = new_sum;
        meta_carry = new_meta_carry;

        // i = 2
        let u = lower.limbs[2].wrapping_mul(MOD::MOD_NEG_INV);
        let (_, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[0], lower.limbs[2], 0);

        // j = 1
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[1], lower.limbs[3], carry);
        lower.limbs[3] = new_limb;
        // j = 2
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[2], upper.limbs[0], carry);
        upper.limbs[0] = new_limb;
        // j = 3
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[3], upper.limbs[1], carry);
        upper.limbs[1] = new_limb;

        let (new_sum, new_meta_carry) = carrying_add(upper.limbs[2], carry, meta_carry);
        upper.limbs[2] = new_sum;
        meta_carry = new_meta_carry;

        // i = 3
        let u = lower.limbs[3].wrapping_mul(MOD::MOD_NEG_INV);
        let (_, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[0], lower.limbs[3], 0);

        // j = 1
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[1], upper.limbs[0], carry);
        upper.limbs[0] = new_limb;
        // j = 2
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[2], upper.limbs[1], carry);
        upper.limbs[1] = new_limb;
        // j = 3
        let (new_limb, carry) = carrying_mul_add(u, MOD::MODULUS.limbs[3], upper.limbs[2], carry);
        upper.limbs[2] = new_limb;

        let (new_sum, new_meta_carry) = carrying_add(upper.limbs[3], carry, meta_carry);
        upper.limbs[3] = new_sum;
        meta_carry = new_meta_carry;

        // Division is simply taking the upper half of the limbs
        // Final reduction (at this point, the value is at most 2 * modulus,
        // so `meta_carry` is either 0 or 1)
        debug_assert!(meta_carry <= 1);

        let (out, borrow) = upper.borrowing_sub(&MOD::MODULUS, 0);

        // The new `borrow = u64::MAX` iff `carry == 0` and `borrow == u64::MAX`.
        let mask = (!meta_carry.wrapping_neg()) & borrow;

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the modulus.
        out.wrapping_add(&MOD::MODULUS.bitand_limb(mask))
    }
}

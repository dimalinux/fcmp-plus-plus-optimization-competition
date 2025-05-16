//! Modular reduction implementation.

use crate::u256::{primitives::carrying_add, U256};

/// Returns `(hi, lo)` such that `hi * R + lo = x * y + z + w`.
#[inline(always)]
#[allow(clippy::cast_possible_truncation)]
const fn muladdcarry(x: u64, y: u64, z: u64, w: u64) -> (u64, u64) {
    let res = (x as u128)
        .wrapping_mul(y as u128)
        .wrapping_add(z as u128)
        .wrapping_add(w as u128);
    ((res >> u64::BITS) as u64, res as u64)
}

/// Algorithm 14.32 in Handbook of Applied Cryptography <https://cacr.uwaterloo.ca/hac/about/chap14.pdf>
pub(crate) const fn montgomery_reduction(
    lower_upper: &(U256, U256),
    modulus: &U256,
    mod_neg_inv: u64,
) -> U256 {
    let (mut lower, mut upper) = *lower_upper;
    let mut meta_carry = 0;

    // i = 0
    let u = lower.limbs[0].wrapping_mul(mod_neg_inv);
    let (carry, _) = muladdcarry(u, modulus.limbs[0], lower.limbs[0], 0);

    // j = 1
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[1], lower.limbs[1], carry);
    lower.limbs[1] = new_limb;
    // j = 2
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[2], lower.limbs[2], carry);
    lower.limbs[2] = new_limb;
    // j = 3
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[3], lower.limbs[3], carry);
    lower.limbs[3] = new_limb;

    let (new_sum, new_meta_carry) = carrying_add(upper.limbs[0], carry, meta_carry);
    upper.limbs[0] = new_sum;
    meta_carry = new_meta_carry;

    // i = 1
    let u = lower.limbs[1].wrapping_mul(mod_neg_inv);
    let (carry, _) = muladdcarry(u, modulus.limbs[0], lower.limbs[1], 0);

    // j = 1
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[1], lower.limbs[2], carry);
    lower.limbs[2] = new_limb;
    // j = 2
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[2], lower.limbs[3], carry);
    lower.limbs[3] = new_limb;
    // j = 3
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[3], upper.limbs[0], carry);
    upper.limbs[0] = new_limb;

    let (new_sum, new_meta_carry) = carrying_add(upper.limbs[1], carry, meta_carry);
    upper.limbs[1] = new_sum;
    meta_carry = new_meta_carry;

    // i = 2
    let u = lower.limbs[2].wrapping_mul(mod_neg_inv);
    let (carry, _) = muladdcarry(u, modulus.limbs[0], lower.limbs[2], 0);

    // j = 1
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[1], lower.limbs[3], carry);
    lower.limbs[3] = new_limb;
    // j = 2
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[2], upper.limbs[0], carry);
    upper.limbs[0] = new_limb;
    // j = 3
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[3], upper.limbs[1], carry);
    upper.limbs[1] = new_limb;

    let (new_sum, new_meta_carry) = carrying_add(upper.limbs[2], carry, meta_carry);
    upper.limbs[2] = new_sum;
    meta_carry = new_meta_carry;

    // i = 3
    let u = lower.limbs[3].wrapping_mul(mod_neg_inv);
    let (carry, _) = muladdcarry(u, modulus.limbs[0], lower.limbs[3], 0);

    // j = 1
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[1], upper.limbs[0], carry);
    upper.limbs[0] = new_limb;
    // j = 2
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[2], upper.limbs[1], carry);
    upper.limbs[1] = new_limb;
    // j = 3
    let (carry, new_limb) = muladdcarry(u, modulus.limbs[3], upper.limbs[2], carry);
    upper.limbs[2] = new_limb;

    let (new_sum, new_meta_carry) = carrying_add(upper.limbs[3], carry, meta_carry);
    upper.limbs[3] = new_sum;
    meta_carry = new_meta_carry;

    // Division is simply taking the upper half of the limbs
    // Final reduction (at this point, the value is at most 2 * modulus,
    // so `meta_carry` is either 0 or 1)

    upper.sub_mod_with_carry(meta_carry, modulus, modulus)
}

use crate::u256::{word, U256};

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
    let mut new_sum;

    let mut i = 0;
    while i < U256::LIMBS {
        let u = lower.limbs[i].wrapping_mul(mod_neg_inv);

        let (mut carry, _) = muladdcarry(u, modulus.limbs[0], lower.limbs[i], 0);
        let mut new_limb;

        let mut j = 1;
        while j < (U256::LIMBS - i) {
            (carry, new_limb) = muladdcarry(u, modulus.limbs[j], lower.limbs[i + j], carry);
            lower.limbs[i + j] = new_limb;
            j += 1;
        }
        while j < U256::LIMBS {
            (carry, new_limb) =
                muladdcarry(u, modulus.limbs[j], upper.limbs[i + j - U256::LIMBS], carry);
            upper.limbs[i + j - U256::LIMBS] = new_limb;
            j += 1;
        }

        (new_sum, meta_carry) = word::adc(upper.limbs[i], carry, meta_carry);
        upper.limbs[i] = new_sum;

        i += 1;
    }

    // Division is simply taking the upper half of the limbs
    // Final reduction (at this point, the value is at most 2 * modulus,
    // so `meta_carry` is either 0 or 1)

    upper.sub_mod_with_carry(meta_carry, modulus, modulus)
}

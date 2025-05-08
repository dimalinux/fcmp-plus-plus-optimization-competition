use super::mul::{mul_montgomery_form, square_montgomery_form};
use crate::bigint::{u256::WORD_BITS, word, Word, U256};

const WINDOW: usize = 4;
const WINDOW_MASK: Word = (1 << WINDOW) - 1;

/// Performs modular exponentiation using Montgomery's ladder.
/// `exponent_bits` represents the number of bits to take into account for the exponent.
///
/// NOTE: this value is leaked in the time pattern.
pub(crate) const fn pow_montgomery_form(
    x: &U256,
    exponent: &U256,
    exponent_bits: usize,
    modulus: &U256,
    r: &U256,
    mod_neg_inv: Word,
) -> U256 {
    multi_exponentiate_montgomery_form_array(
        &[(*x, *exponent)],
        exponent_bits,
        modulus,
        r,
        mod_neg_inv,
    )
}

const fn multi_exponentiate_montgomery_form_array(
    bases_and_exponents: &[(U256, U256); 1],
    exponent_bits: usize,
    modulus: &U256,
    r: &U256,
    mod_neg_inv: Word,
) -> U256 {
    if exponent_bits == 0 {
        return *r; // 1 in Montgomery form
    }

    let mut powers_and_exponents = [([U256::ZERO; 1 << WINDOW], U256::ZERO); 1];

    let mut i = 0;
    while i < 1 {
        let (base, exponent) = bases_and_exponents[i];
        powers_and_exponents[i] = (compute_powers(&base, modulus, r, mod_neg_inv), exponent);
        i += 1;
    }

    multi_exponentiate_montgomery_form_internal(
        &powers_and_exponents,
        exponent_bits,
        modulus,
        r,
        mod_neg_inv,
    )
}

const fn compute_powers(
    x: &U256,
    modulus: &U256,
    r: &U256,
    mod_neg_inv: Word,
) -> [U256; 1 << WINDOW] {
    // powers[i] contains x^i
    let mut powers = [*r; 1 << WINDOW];
    powers[1] = *x;

    let mut i = 2;
    while i < powers.len() {
        powers[i] = mul_montgomery_form(&powers[i - 1], x, modulus, mod_neg_inv);
        i += 1;
    }

    powers
}

const fn multi_exponentiate_montgomery_form_internal(
    powers_and_exponents: &[([U256; 1 << WINDOW], U256)],
    exponent_bits: usize,
    modulus: &U256,
    r: &U256,
    mod_neg_inv: Word,
) -> U256 {
    let starting_limb = (exponent_bits - 1) / WORD_BITS;
    let starting_bit_in_limb = (exponent_bits - 1) % WORD_BITS;
    let starting_window = starting_bit_in_limb / WINDOW;
    let starting_window_mask = (1 << (starting_bit_in_limb % WINDOW + 1)) - 1;

    let mut z = *r; // 1 in Montgomery form

    let mut limb_num = starting_limb + 1;
    while limb_num > 0 {
        limb_num -= 1;

        let mut window_num = if limb_num == starting_limb {
            starting_window + 1
        } else {
            WORD_BITS / WINDOW
        };
        while window_num > 0 {
            window_num -= 1;

            if limb_num != starting_limb || window_num != starting_window {
                let mut i = 0;
                while i < WINDOW {
                    i += 1;
                    z = square_montgomery_form(&z, modulus, mod_neg_inv);
                }
            }

            let mut i = 0;
            while i < powers_and_exponents.len() {
                let (powers, exponent) = powers_and_exponents[i];
                let w = exponent.as_words()[limb_num];
                let mut idx = (w >> (window_num * WINDOW)) & WINDOW_MASK;

                if limb_num == starting_limb && window_num == starting_window {
                    idx &= starting_window_mask;
                }

                // Constant-time lookup in the array of powers
                let mut power = powers[0];
                let mut j = 1;
                while j < 1 << WINDOW {
                    let choice = word::ct_eq(j as Word, idx);
                    power = U256::ct_select(&power, &powers[j], choice);
                    j += 1;
                }

                z = mul_montgomery_form(&z, &power, modulus, mod_neg_inv);
                i += 1;
            }
        }
    }

    z
}

use super::reduction::montgomery_reduction;
use crate::bigint::{Word, U256};

pub(crate) const fn mul_montgomery_form(
    a: &U256,
    b: &U256,
    modulus: &U256,
    mod_neg_inv: Word,
) -> U256 {
    let product = a.mul_wide(b);
    montgomery_reduction(&product, modulus, mod_neg_inv)
}

pub(crate) const fn square_montgomery_form(a: &U256, modulus: &U256, mod_neg_inv: Word) -> U256 {
    let product = a.square_wide();
    montgomery_reduction(&product, modulus, mod_neg_inv)
}

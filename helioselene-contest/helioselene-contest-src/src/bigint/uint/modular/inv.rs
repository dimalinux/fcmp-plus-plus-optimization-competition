use crate::bigint::{montgomery_reduction, CtChoice, Limb, U256};

pub(crate) const fn inv_montgomery_form(
    x: &U256,
    modulus: &U256,
    r3: &U256,
    mod_neg_inv: Limb,
) -> (U256, CtChoice) {
    let (inverse, is_some) = x.inv_odd_mod(modulus);
    (
        montgomery_reduction(&inverse.mul_wide(r3), modulus, mod_neg_inv),
        is_some,
    )
}

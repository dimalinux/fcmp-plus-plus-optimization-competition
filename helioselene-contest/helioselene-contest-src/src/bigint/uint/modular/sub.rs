use crate::bigint::U256;

pub(crate) const fn sub_montgomery_form(a: &U256, b: &U256, modulus: &U256) -> U256 {
    a.sub_mod(b, modulus)
}

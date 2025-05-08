use crate::bigint::U256;

pub(crate) const fn add_montgomery_form(a: &U256, b: &U256, modulus: &U256) -> U256 {
    a.add_mod(b, modulus)
}

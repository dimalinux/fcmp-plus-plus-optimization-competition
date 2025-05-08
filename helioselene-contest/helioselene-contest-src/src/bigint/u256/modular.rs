mod reduction;

/// Implements `Residue`s, supporting modular arithmetic with a constant modulus.
pub mod constant_mod;

mod add;
mod inv;
mod mul;
mod pow;
mod sub;

pub(crate) use reduction::montgomery_reduction;

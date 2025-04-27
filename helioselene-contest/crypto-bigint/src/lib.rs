#![no_std]
//! ## Usage
//!
//! This crate defines a [`Uint`] type which is const generic around an inner
//! [`Limb`] array, where a [`Limb`] is a newtype for a word-sized integer.
//! Thus large integers are represented as arrays of smaller integers which
//! are sized appropriately for the CPU, giving us some assurances of how
//! arithmetic operations over those smaller integers will behave.
//!
//! To obtain appropriately sized integers regardless of what a given CPU's
//! word size happens to be, a number of portable type aliases are provided for
//! integer sizes commonly used in cryptography, for example,
//! [`U256`], [`U512`]
//!
//! ### `const fn` usage
//!
//! The [`Uint`] type provides a number of `const fn` inherent methods which
//! can be used for initializing and performing arithmetic on big integers in
//! const contexts:
//!
//! ```
//! use crypto_bigint::U256;
//!
//! // Parse a constant from a big endian hexadecimal string.
//! pub const MODULUS: U256 =
//!     U256::from_be_hex("ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551");
//!
//! // Compute `MODULUS` shifted right by 1 at compile time
//! pub const MODULUS_SHR1: U256 = MODULUS.shr_vartime(1);
//! ```
//!
//! Note that large constant computations may accidentally trigger a the `const_eval_limit` of the compiler.
//! The current way to deal with this problem is to either simplify this computation,
//! or increase the compiler's limit (currently a nightly feature).
//! One can completely remove the compiler's limit using:
//! ```ignore
//! #![feature(const_eval_limit)]
//! #![const_eval_limit = "0"]
//! ```
//!
//! ### Trait-based usage
//!
//! The [`Uint`] type itself does not implement the standard arithmetic traits
//! such as [`Add`], [`Sub`], [`Mul`], and [`Div`].
//!
//! ### Modular arithmetic
//!
//! This library has initial support for modular arithmetic in the form of the
//! [`AddMod`], [`SubMod`], [`NegMod`], and [`MulMod`] traits, as well as the
//! support for the [`Rem`] trait when used with a [`NonZero`] operand.
//!
//! ```
//! use crypto_bigint::{AddMod, U256};
//!
//! // mod 3
//! let modulus = U256::from(3u8);
//!
//! // 1 + 1 mod 3 = 2
//! let a = U256::ONE.add_mod(&U256::ONE, &modulus);
//! assert_eq!(a, U256::from(2u8));
//!
//! // 2 + 1 mod 3 = 0
//! let b = a.add_mod(&U256::ONE, &modulus);
//! assert_eq!(b, U256::ZERO);
//! ```
//!
//! It also supports modular arithmetic over constant moduli using `Residue`,
//! and over moduli set at runtime using `DynResidue`.
//! That includes modular exponentiation and multiplicative inverses.
//! These features are described in the [`modular`] module.
//!
//!
//! [`Add`]: core::ops::Add
//! [`Div`]: core::ops::Div
//! [`Mul`]: core::ops::Mul
//! [`Rem`]: core::ops::Rem
//! [`Sub`]: core::ops::Sub

mod ct_choice;
mod limb;
mod non_zero;
mod traits;
mod uint;

pub use crate::{
    ct_choice::CtChoice,
    limb::{Limb, WideWord, Word},
    non_zero::NonZero,
    traits::*,
    uint::*,
};
pub use subtle;
pub use zeroize;

/// Import prelude for this crate: includes important traits.
pub mod prelude {
    pub use crate::traits::*;
}

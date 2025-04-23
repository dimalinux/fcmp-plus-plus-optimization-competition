//! Implementation of constant-time division via reciprocal precomputation, as described in
//! "Improved Division by Invariant Integers" by Niels Möller and Torbjorn Granlund
//! (DOI: 10.1109/TC.2010.143, <https://gmplib.org/~tege/division-paper.pdf>).
use subtle::{Choice, ConditionallySelectable};

use crate::Word;

/// A pre-calculated reciprocal for division by a single limb.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reciprocal {
    divisor_normalized: Word,
    shift: u32,
    reciprocal: Word,
}

impl Reciprocal {
    /// Returns a default instance of this object.
    /// It is a self-consistent `Reciprocal` that will not cause panics in functions that take it.
    ///
    /// NOTE: intended for using it as a placeholder during compile-time array generation,
    /// don't rely on the contents.
    pub const fn default() -> Self {
        Self {
            divisor_normalized: Word::MAX,
            shift: 0,
            // The result of calling `reciprocal(Word::MAX)`
            // This holds both for 32- and 64-bit versions.
            reciprocal: 1,
        }
    }
}

impl ConditionallySelectable for Reciprocal {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self {
            divisor_normalized: Word::conditional_select(
                &a.divisor_normalized,
                &b.divisor_normalized,
                choice,
            ),
            shift: u32::conditional_select(&a.shift, &b.shift, choice),
            reciprocal: Word::conditional_select(&a.reciprocal, &b.reciprocal, choice),
        }
    }
}

// `CtOption.map()` needs this; for some reason it doesn't use the value it already has
// for the `None` branch.
impl Default for Reciprocal {
    fn default() -> Self {
        Self::default()
    }
}

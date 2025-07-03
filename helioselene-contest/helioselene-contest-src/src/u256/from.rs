//! const `From`-like conversions for [`U256`].

use crate::u256::U256;

impl U256 {
    /// Create a [`Uint`] from a `u64` (const-friendly)
    // TODO: replace with `const impl From<u64>` when stable
    pub(crate) const fn from_u64(n: u64) -> Self {
        Self {
            limbs: [n, 0, 0, 0],
        }
    }

    /// Create a [`Uint`] from a `u128` (const-friendly)
    // TODO: replace with `const impl From<u128>` when stable
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn from_u128(n: u128) -> Self {
        Self {
            limbs: [n as u64, (n >> 64) as u64, 0, 0],
        }
    }
}

//! [`Uint`] bitwise right shift operations.

use super::Uint;
use crate::{limb::HI_BIT, CtChoice, Limb};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `self >> 1` in constant-time, returning [`CtChoice::TRUE`] if the overflowing bit
    /// was set, and [`CtChoice::FALSE`] otherwise.
    pub(crate) const fn shr_1(&self) -> (Self, CtChoice) {
        let mut shifted_bits = [0; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            shifted_bits[i] = self.limbs[i].0 >> 1;
            i += 1;
        }

        let mut carry_bits = [0; LIMBS];
        let mut i = 0;
        while i < LIMBS {
            carry_bits[i] = self.limbs[i].0 << HI_BIT;
            i += 1;
        }

        let mut limbs = [Limb(0); LIMBS];

        let mut i = 0;
        while i < (LIMBS - 1) {
            limbs[i] = Limb(shifted_bits[i] | carry_bits[i + 1]);
            i += 1;
        }
        limbs[LIMBS - 1] = Limb(shifted_bits[LIMBS - 1]);

        debug_assert!(carry_bits[LIMBS - 1] == 0 || carry_bits[LIMBS - 1] == (1 << HI_BIT));
        (
            Uint::new(limbs),
            CtChoice::from_lsb(carry_bits[0] >> HI_BIT),
        )
    }

    /// Computes `self >> n`.
    ///
    /// NOTE: this operation is variable time with respect to `n` *ONLY*.
    ///
    /// When used with a fixed `n`, this function is constant-time with respect
    /// to `self`.
    #[inline(always)]
    pub const fn shr_vartime(&self, shift: usize) -> Self {
        let full_shifts = shift / Limb::BITS;
        let small_shift = shift & (Limb::BITS - 1);
        let mut limbs = [Limb::ZERO; LIMBS];

        if shift > Limb::BITS * LIMBS {
            return Self { limbs };
        }

        let n = LIMBS - full_shifts;
        let mut i = 0;

        if small_shift == 0 {
            while i < n {
                limbs[i] = Limb(self.limbs[i + full_shifts].0);
                i += 1;
            }
        } else {
            while i < n {
                let mut lo = self.limbs[i + full_shifts].0 >> small_shift;

                if i < (LIMBS - 1) - full_shifts {
                    lo |= self.limbs[i + full_shifts + 1].0 << (Limb::BITS - small_shift);
                }

                limbs[i] = Limb(lo);
                i += 1;
            }
        }

        Self { limbs }
    }

    /// Computes a right shift on a wide input as `(lo, hi)`.
    ///
    /// NOTE: this operation is variable time with respect to `n` *ONLY*.
    ///
    /// When used with a fixed `n`, this function is constant-time with respect
    /// to `self`.
    #[inline(always)]
    pub const fn shr_vartime_wide(lower_upper: (Self, Self), n: usize) -> (Self, Self) {
        let (mut lower, upper) = lower_upper;
        let new_upper = upper.shr_vartime(n);
        lower = lower.shr_vartime(n);
        if n >= Self::BITS {
            lower = lower.bitor(&upper.shr_vartime(n - Self::BITS));
        } else {
            lower = lower.bitor(&upper.shl_vartime(Self::BITS - n));
        }

        (lower, new_upper)
    }
}

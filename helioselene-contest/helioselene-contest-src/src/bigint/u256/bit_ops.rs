use crate::bigint::{
    ct_choice::CtChoice,
    u256::WORD_BITS,
    word::{ct_is_nonzero, ct_select, Word},
    U256,
};

impl U256 {
    /// Calculate the number of bits needed to represent this number.
    pub const fn bits_vartime(&self) -> usize {
        let mut i = Self::LIMBS - 1;
        while i > 0 && self.limbs[i] == 0 {
            i -= 1;
        }

        let limb = self.limbs[i];
        WORD_BITS * (i + 1) - (limb.leading_zeros() as usize)
    }

    #[inline(always)]
    pub const fn bitand(&self, rhs: &Self) -> Self {
        let mut limbs = [0; Self::LIMBS];
        let mut i = 0;

        while i < Self::LIMBS {
            limbs[i] = self.limbs[i] & rhs.limbs[i];
            i += 1;
        }

        Self { limbs }
    }

    /// Computes bitwise `a & b`.
    #[inline(always)]
    pub const fn bitor(&self, rhs: &Self) -> Self {
        let mut limbs = [0; Self::LIMBS];
        let mut i = 0;

        while i < Self::LIMBS {
            limbs[i] = self.limbs[i] | rhs.limbs[i];
            i += 1;
        }

        Self { limbs }
    }

    /// Computes `self << shift` where `0 <= shift < WORD_BITS`,
    /// returning the result and the carry.
    #[inline(always)]
    pub(crate) const fn shl_limb(&self, n: usize) -> (Self, Word) {
        let mut limbs = [0; Self::LIMBS];

        let nz = ct_is_nonzero(n as Word);
        let lshift = n as Word;
        let rshift = ct_select(0, (WORD_BITS - n) as Word, nz);
        let carry = ct_select(
            0,
            self.limbs[Self::LIMBS - 1].wrapping_shr(Word::BITS - n as u32),
            nz,
        );

        let mut i = Self::LIMBS - 1;
        while i > 0 {
            let mut limb = self.limbs[i] << lshift;
            let hi = self.limbs[i - 1] >> rshift;
            limb |= nz.if_true(hi);
            limbs[i] = limb;
            i -= 1
        }
        limbs[0] = self.limbs[0] << lshift;

        (Self::new(limbs), carry)
    }

    /// Computes `self << shift`.
    ///
    /// NOTE: this operation is variable time with respect to `n` *ONLY*.
    ///
    /// When used with a fixed `n`, this function is constant-time with respect
    /// to `self`.
    #[inline(always)]
    pub const fn shl_vartime(&self, n: usize) -> Self {
        let mut limbs = [0; Self::LIMBS];

        if n >= WORD_BITS * Self::LIMBS {
            return Self { limbs };
        }

        let shift_num = n / WORD_BITS;
        let rem = n % WORD_BITS;

        let mut i = Self::LIMBS;
        while i > shift_num {
            i -= 1;
            limbs[i] = self.limbs[i - shift_num];
        }

        let (new_lower, _carry) = (Self { limbs }).shl_limb(rem);
        new_lower
    }

    /// Computes a left shift on a wide input as `(lo, hi)`.
    ///
    /// NOTE: this operation is variable time with respect to `n` *ONLY*.
    ///
    /// When used with a fixed `n`, this function is constant-time with respect
    /// to `self`.
    #[inline(always)]
    pub const fn shl_vartime_wide(lower_upper: (Self, Self), n: usize) -> (Self, Self) {
        let (lower, mut upper) = lower_upper;
        let new_lower = lower.shl_vartime(n);
        upper = upper.shl_vartime(n);
        if n >= Self::BITS {
            upper = upper.bitor(&lower.shl_vartime(n - Self::BITS));
        } else {
            upper = upper.bitor(&lower.shr_vartime(Self::BITS - n));
        }

        (new_lower, upper)
    }

    /// Computes `self >> 1` in constant-time, returning [`CtChoice::TRUE`] if the overflowing bit
    /// was set, and [`CtChoice::FALSE`] otherwise.
    pub(crate) const fn shr_1(&self) -> (Self, CtChoice) {
        let mut shifted_bits = [0; Self::LIMBS];
        let mut i = 0;
        while i < Self::LIMBS {
            shifted_bits[i] = self.limbs[i] >> 1;
            i += 1;
        }

        let mut carry_bits = [0; Self::LIMBS];
        let mut i = 0;
        while i < Self::LIMBS {
            carry_bits[i] = self.limbs[i] << (WORD_BITS - 1);
            i += 1;
        }

        let mut limbs = [0; Self::LIMBS];

        let mut i = 0;
        while i < (Self::LIMBS - 1) {
            limbs[i] = shifted_bits[i] | carry_bits[i + 1];
            i += 1;
        }
        limbs[Self::LIMBS - 1] = shifted_bits[Self::LIMBS - 1];

        debug_assert!(
            carry_bits[Self::LIMBS - 1] == 0
                || carry_bits[Self::LIMBS - 1] == (1 << (Word::BITS - 1))
        );
        (
            Self::new(limbs),
            CtChoice::from_lsb(carry_bits[0] >> (Word::BITS - 1)),
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
        let full_shifts = shift / WORD_BITS;
        let small_shift = shift & (WORD_BITS - 1);
        let mut limbs = [0; Self::LIMBS];

        if shift > WORD_BITS * Self::LIMBS {
            return Self { limbs };
        }

        let n = Self::LIMBS - full_shifts;
        let mut i = 0;

        if small_shift == 0 {
            while i < n {
                limbs[i] = self.limbs[i + full_shifts];
                i += 1;
            }
        } else {
            while i < n {
                let mut lo = self.limbs[i + full_shifts] >> small_shift;

                if i < (Self::LIMBS - 1) - full_shifts {
                    lo |= self.limbs[i + full_shifts + 1] << (WORD_BITS - small_shift);
                }

                limbs[i] = lo;
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

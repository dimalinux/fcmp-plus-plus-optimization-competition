use crate::u256::{ct_choice::CtChoice, U256};

impl U256 {
    pub(crate) const fn bitand_limb(&self, rhs: u64) -> Self {
        Self {
            limbs: [
                self.limbs[0] & rhs,
                self.limbs[1] & rhs,
                self.limbs[2] & rhs,
                self.limbs[3] & rhs,
            ],
        }
    }

    /// Computes `self >> 1` in constant-time, returning [`CtChoice::TRUTHY`] if the overflowing bit
    /// was set, and [`CtChoice::FALSE`] otherwise.
    pub(crate) const fn shr_1(&self) -> (Self, CtChoice) {
        (
            Self::new([
                (self.limbs[0] >> 1) | (self.limbs[1] << 63),
                (self.limbs[1] >> 1) | (self.limbs[2] << 63),
                (self.limbs[2] >> 1) | (self.limbs[3] << 63),
                self.limbs[3] >> 1,
            ]),
            CtChoice::from_lsb(self.limbs[0] & 1),
        )
    }

    /// Computes `self << 1` in constant-time, returning the result and the carry-out
    /// which is always zero or one.
    pub(crate) const fn shl_1(&self) -> (Self, u64) {
        let carry_out = self.limbs[3] >> 63;
        (
            Self::new([
                self.limbs[0] << 1,
                (self.limbs[1] << 1) | (self.limbs[0] >> 63),
                (self.limbs[2] << 1) | (self.limbs[1] >> 63),
                (self.limbs[3] << 1) | (self.limbs[2] >> 63),
            ]),
            carry_out,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shr_1_lsb_carry() {
        // lowest limb has bit 0 set → shifted out as carry
        let x = U256::new([1, 0, 0, 0]);
        let (y, carry) = x.shr_1();
        assert_eq!(y, U256::ZERO);
        assert!(carry.is_true_vartime());
    }

    #[test]
    fn shr_1_cross_limb() {
        // bit 0 in limb 1 moves into MSB of limb 0
        let x = U256::new([0, 1, 0, 0]);
        let (y, carry) = x.shr_1();
        let expected = U256::new([1 << (u64::BITS - 1), 0, 0, 0]);
        assert_eq!(y, expected);
        assert!(!carry.is_true_vartime());
    }

    #[test]
    fn shr_1_no_sign_extension() {
        // bit 0 in limb 1 moves into MSB of limb 0
        let x = U256::new([0, 0, 0, u64::MAX]);
        let (y, carry) = x.shr_1();
        let expected = U256::new([0, 0, 0x8000000000000000, 0x7FFFFFFFFFFFFFFF]);
        assert_eq!(y, expected);
        assert!(!carry.is_true_vartime());
    }
}

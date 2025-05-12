use crate::u256::{ct_choice::CtChoice, U256};

impl U256 {
    /// Negates based on `choice` by wrapping the integer.
    pub(crate) const fn conditional_wrapping_neg(&self, choice: CtChoice) -> U256 {
        Self::ct_select(self, &self.wrapping_neg(), choice)
    }

    /// Perform wrapping negation.
    pub(crate) const fn wrapping_neg(&self) -> Self {
        let mut carry = 1;

        let r0 = (!self.limbs[0] as u128) + carry;
        carry = r0 >> u64::BITS;

        let r1 = (!self.limbs[1] as u128) + carry;
        carry = r1 >> u64::BITS;

        let r2 = (!self.limbs[2] as u128) + carry;
        carry = r2 >> u64::BITS;

        let r3 = (!self.limbs[3] as u128) + carry;

        let ret = [r0 as u64, r1 as u64, r2 as u64, r3 as u64];
        Self::new(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::U256;

    #[test]
    fn test_wrapping_neg_zero() {
        // flip all the bits, add one, and you're back to zero
        assert_eq!(U256::ZERO.wrapping_neg(), U256::ZERO);
    }

    #[test]
    fn test_wrapping_neg_one() {
        assert_eq!(U256::ONE.wrapping_neg(), U256::MAX);
        assert_eq!(U256::MAX.wrapping_neg(), U256::ONE);
    }
}

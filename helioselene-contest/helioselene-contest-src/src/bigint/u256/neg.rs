use crate::bigint::{
    ct_choice::CtChoice,
    u256::{Word, WORD_BITS},
    word::WideWord,
    U256,
};

impl U256 {
    /// Negates based on `choice` by wrapping the integer.
    pub(crate) const fn conditional_wrapping_neg(&self, choice: CtChoice) -> U256 {
        Self::ct_select(self, &self.wrapping_neg(), choice)
    }

    /// Perform wrapping negation.
    pub const fn wrapping_neg(&self) -> Self {
        let mut ret = [0; Self::LIMBS];
        let mut carry = 1;
        let mut i = 0;
        while i < Self::LIMBS {
            let r = (!self.limbs[i] as WideWord) + carry;
            ret[i] = r as Word;
            carry = r >> WORD_BITS;
            i += 1;
        }
        Self::new(ret)
    }
}

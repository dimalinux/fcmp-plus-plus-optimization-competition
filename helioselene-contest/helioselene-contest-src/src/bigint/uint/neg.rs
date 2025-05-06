use crate::bigint::{
    ct_choice::CtChoice,
    limb::WideWord,
    uint::{Limb, Word},
    U256,
};

impl U256 {
    /// Negates based on `choice` by wrapping the integer.
    pub(crate) const fn conditional_wrapping_neg(&self, choice: CtChoice) -> U256 {
        Self::ct_select(self, &self.wrapping_neg(), choice)
    }

    /// Perform wrapping negation.
    pub const fn wrapping_neg(&self) -> Self {
        let mut ret = [Limb::ZERO; Self::LIMBS];
        let mut carry = 1;
        let mut i = 0;
        while i < Self::LIMBS {
            let r = (!self.limbs[i].0 as WideWord) + carry;
            ret[i] = Limb(r as Word);
            carry = r >> Limb::BITS;
            i += 1;
        }
        Self::new(ret)
    }
}

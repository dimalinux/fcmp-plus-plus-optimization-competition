//! Limb multiplication

use crate::{Limb, WideWord, Word};

impl Limb {
    /// Computes `self + (b * c) + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn mac(self, b: Limb, c: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = b.0 as WideWord;
        let c = c.0 as WideWord;
        let carry = carry.0 as WideWord;
        let ret = a + (b * c) + carry;
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }
}

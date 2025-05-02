//! [`Uint`] addition modulus operations.

use crate::bigint::{limb::Limb, uint::Uint};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `self + rhs mod p`.
    ///
    /// Assumes `self + rhs` as unbounded integer is `< 2p`.
    pub const fn add_mod(&self, rhs: &Uint<LIMBS>, p: &Uint<LIMBS>) -> Uint<LIMBS> {
        let (w, carry) = self.adc(rhs, Limb::ZERO);

        // Attempt to subtract the modulus, to ensure the result is in the field.
        let (w, borrow) = w.sbb(p, Limb::ZERO);
        let (_, borrow) = carry.sbb(Limb::ZERO, borrow);

        // If underflow occurred on the final limb, borrow = 0xfff...fff, otherwise
        // borrow = 0x000...000. Thus, we use it as a mask to conditionally add the
        // modulus.
        let mask = Uint::from_words([borrow.0; LIMBS]);

        w.wrapping_add(&p.bitand(&mask))
    }
}

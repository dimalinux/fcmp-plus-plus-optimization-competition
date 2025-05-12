use crate::u256::{ct_choice::CtChoice, U256};

impl U256 {
    /// Computes the multiplicative inverse of `self` mod `modulus`, where `modulus` is odd.
    /// Returns `(inverse, CtChoice::TRUTHY)` if an inverse exists,
    /// otherwise `(undefined, CtChoice::FALSY)`.
    pub(crate) const fn inv_odd_mod(&self, modulus: &Self) -> (Self, CtChoice) {
        debug_assert!(modulus.ct_is_odd().is_true_vartime());

        let mut a = *self;

        let mut u = Self::ONE;
        let mut v = Self::ZERO;

        let mut b = *modulus;

        // `BIT_SIZE` can be anything >= `self.bits()` + `modulus.bits()`, setting to the minimum.
        const BIT_SIZE: usize = U256::BITS * 2;

        let mut m1hp = *modulus;
        let (m1hp_new, carry) = m1hp.shr_1();
        debug_assert!(carry.is_true_vartime());
        m1hp = m1hp_new.wrapping_add(&Self::ONE);

        let mut i = 0;
        while i < BIT_SIZE {
            debug_assert!(b.ct_is_odd().is_true_vartime());

            let self_odd = a.ct_is_odd();

            // Set `self -= b` if `self` is odd.
            let (new_a, swap) = a.conditional_wrapping_sub(&b, self_odd);
            // Set `b += self` if `swap` is true.
            b = Self::ct_select(&b, &b.wrapping_add(&new_a), swap);
            // Negate `self` if `swap` is true.
            a = new_a.conditional_wrapping_neg(swap);

            let (new_u, new_v) = Self::ct_swap(&u, &v, swap);
            let (new_u, cy) = new_u.conditional_wrapping_sub(&new_v, self_odd);
            let (new_u, cyy) = new_u.conditional_wrapping_add(modulus, cy);
            debug_assert!(cy.is_true_vartime() == cyy.is_true_vartime());

            let (new_a, overflow) = a.shr_1();
            debug_assert!(!overflow.is_true_vartime());
            let (new_u, cy) = new_u.shr_1();
            let (new_u, cy) = new_u.conditional_wrapping_add(&m1hp, cy);
            debug_assert!(!cy.is_true_vartime());

            a = new_a;
            u = new_u;
            v = new_v;

            i += 1;
        }

        debug_assert!(!a.ct_is_nonzero().is_true_vartime());

        (v, Self::ct_eq(&b, &Self::ONE))
    }
}

//! Multiplicative inverses of integers in Montgomery form with a constant modulus.

use core::marker::PhantomData;

use crate::u256::{ct_choice::CtChoice, MontyForm, MontyParams, U256};

// TODO: Newer versions of crypto-bigint have a different invert implementation,
//       Test to see if it is faster.

impl<MOD: MontyParams> MontyForm<MOD> {
    const M1HP: U256 = MOD::MODULUS.shr_1().0.wrapping_add(&U256::ONE);

    /// Computes the multiplicative inverse of `self` mod `modulus`, where `modulus` is odd.
    /// Returns `(inverse, CtChoice::TRUTHY)` if an inverse exists,
    /// otherwise `(undefined, CtChoice::FALSY)`.
    const fn inv_odd_mod(&self) -> (U256, CtChoice) {
        debug_assert!(MOD::MODULUS.ct_is_odd().is_true_vartime());

        let mut a = self.montgomery_form;
        let mut u = U256::ONE;
        let mut v = U256::ZERO;
        let mut b = MOD::MODULUS;

        // `BIT_SIZE` can be anything >= `self.bits()` + `modulus.bits()`, setting to the minimum.
        const BIT_SIZE: usize = U256::BITS * 2;

        let mut i = 0;
        while i < BIT_SIZE {
            debug_assert!(b.ct_is_odd().is_true_vartime());

            let self_odd = a.ct_is_odd();

            // Set `self -= b` if `self` is odd.
            let (new_a, swap) = a.conditional_wrapping_sub(&b, self_odd);
            // Set `b += self` if `swap` is true.
            b = U256::ct_select(&b, &b.wrapping_add(&new_a), swap);
            // Negate `self` if `swap` is true.
            a = new_a.conditional_wrapping_neg(swap);

            let (new_u, new_v) = U256::ct_swap(&u, &v, swap);
            let (new_u, cy) = new_u.conditional_wrapping_sub(&new_v, self_odd);
            let (new_u, cyy) = new_u.conditional_wrapping_add(&MOD::MODULUS, cy);
            debug_assert!(cy.is_true_vartime() == cyy.is_true_vartime());

            let (new_a, overflow) = a.shr_1();
            debug_assert!(!overflow.is_true_vartime());
            let (new_u, cy) = new_u.shr_1();
            let (new_u, cy) = new_u.conditional_wrapping_add(&Self::M1HP, cy);
            debug_assert!(!cy.is_true_vartime());

            a = new_a;
            u = new_u;
            v = new_v;

            i += 1;
        }

        debug_assert!(!a.ct_is_nonzero().is_true_vartime());

        (v, U256::ct_eq(&b, &U256::ONE))
    }

    /// Computes `self^-1` representing the multiplicative inverse of `self`,
    /// i.e. `self * self^-1 = 1`.
    ///
    /// If the number was invertible, the second element of the tuple is the truthy value,
    /// otherwise it is the falsy value (in which case the first element's value is unspecified).
    pub(crate) const fn invert(&self) -> (Self, CtChoice) {
        // Compute the inverse in Montgomery form.
        let (inverse, is_some) = self.inv_odd_mod();

        // Multiply with R3 and reduce in Montgomery form.
        let montgomery_form = Self::montgomery_reduction(&Self::mul_wide(&inverse, &MOD::R3));

        let value = Self {
            montgomery_form,
            phantom: PhantomData,
        };

        (value, is_some)
    }
}

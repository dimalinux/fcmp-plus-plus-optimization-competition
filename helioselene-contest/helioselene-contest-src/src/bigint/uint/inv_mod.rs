use super::Uint;
use crate::bigint::{ct_choice::CtChoice, U256};

impl U256 {
    /// Computes 1/`self` mod `2^k`.
    /// This method is constant-time w.r.t. `self` but not `k`.
    ///
    /// Conditions: `self` < 2^k and `self` must be odd
    pub const fn inv_mod2k_vartime(&self, k: usize) -> Self {
        // Using the Algorithm 3 from "A Secure Algorithm for Inversion Modulo 2k"
        // by Sadiel de la Fe and Carles Ferrer.
        // See <https://www.mdpi.com/2410-387X/2/3/23>.

        // Note that we are not using Alrgorithm 4, since we have a different approach
        // of enforcing constant-timeness w.r.t. `self`.

        let mut x = Self::ZERO; // keeps `x` during iterations
        let mut b = Self::ONE; // keeps `b_i` during iterations
        let mut i = 0;

        while i < k {
            // X_i = b_i mod 2
            let x_i = b.limbs[0].0 & 1;
            let x_i_choice = CtChoice::from_lsb(x_i);
            // b_{i+1} = (b_i - a * X_i) / 2
            b = Self::ct_select(&b, &b.wrapping_sub(self), x_i_choice).shr_vartime(1);
            // Store the X_i bit in the result (x = x | (1 << X_i))
            x = x.bitor(&Uint::from_word(x_i).shl_vartime(i));

            i += 1;
        }

        x
    }

    /// Computes the multiplicative inverse of `self` mod `modulus`, where `modulus` is odd.
    /// In other words `self^-1 mod modulus`.
    /// `bits` and `modulus_bits` are the bounds on the bit size
    /// of `self` and `modulus`, respectively
    /// (the inversion speed will be proportional to `bits + modulus_bits`).
    /// The second element of the tuple is the truthy value if an inverse exists,
    /// otherwise it is a falsy value.
    ///
    /// **Note:** variable time in `bits` and `modulus_bits`.
    ///
    /// The algorithm is the same as in GMP 6.2.1's `mpn_sec_invert`.
    pub(crate) const fn inv_odd_mod_bounded(
        &self,
        modulus: &Self,
        bits: usize,
        modulus_bits: usize,
    ) -> (Self, CtChoice) {
        debug_assert!(modulus.ct_is_odd().is_true_vartime());

        let mut a = *self;

        let mut u = Uint::ONE;
        let mut v = Uint::ZERO;

        let mut b = *modulus;

        // `bit_size` can be anything >= `self.bits()` + `modulus.bits()`, setting to the minimum.
        let bit_size = bits + modulus_bits;

        let mut m1hp = *modulus;
        let (m1hp_new, carry) = m1hp.shr_1();
        debug_assert!(carry.is_true_vartime());
        m1hp = m1hp_new.wrapping_add(&Uint::ONE);

        let mut i = 0;
        while i < bit_size {
            debug_assert!(b.ct_is_odd().is_true_vartime());

            let self_odd = a.ct_is_odd();

            // Set `self -= b` if `self` is odd.
            let (new_a, swap) = a.conditional_wrapping_sub(&b, self_odd);
            // Set `b += self` if `swap` is true.
            b = Uint::ct_select(&b, &b.wrapping_add(&new_a), swap);
            // Negate `self` if `swap` is true.
            a = new_a.conditional_wrapping_neg(swap);

            let (new_u, new_v) = Uint::ct_swap(&u, &v, swap);
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

        (v, Uint::ct_eq(&b, &Uint::ONE))
    }

    /// Computes the multiplicative inverse of `self` mod `modulus`, where `modulus` is odd.
    /// Returns `(inverse, CtChoice::TRUE)` if an inverse exists,
    /// otherwise `(undefined, CtChoice::FALSE)`.
    pub(crate) const fn inv_odd_mod(&self, modulus: &Self) -> (Self, CtChoice) {
        self.inv_odd_mod_bounded(modulus, Self::BITS, Self::BITS)
    }
}

use core::marker::PhantomData;

use crate::u256::{CtChoice, FixedExponent, MontyForm, MontyParams, U256};

impl<MOD: MontyParams> MontyForm<MOD> {
    const P_MOD_4_IS_3: bool = MOD::MODULUS.least_significant_byte() & 0b11 == 3;
    const P_MOD_8_IS_5: bool = MOD::MODULUS.least_significant_byte() & 0b111 == 5;

    pub(crate) const fn sqrt(&self) -> (Self, CtChoice) {
        if Self::P_MOD_4_IS_3 {
            debug_assert!(MOD::MOD_PLUS_1_DIV_4.ct_is_nonzero().is_true_vartime());
            return self.sqrt_p_mod_4_is_3();
        }

        if Self::P_MOD_8_IS_5 {
            debug_assert!(MOD::MOD_3_8.ct_is_nonzero().is_true_vartime());
            debug_assert!(MOD::SQRT_M1
                .montgomery_form
                .ct_is_nonzero()
                .is_true_vartime());
            return self.sqrt_p_mod_8_is_5();
        }

        unimplemented!();
    }

    #[inline(always)]
    const fn sqrt_p_mod_4_is_3(&self) -> (Self, CtChoice) {
        // Using recipe from RFC-8032 sqrt4k3.
        struct ModPlusOneDivFour<MOD: MontyParams>(PhantomData<MOD>);
        impl<MOD: MontyParams> FixedExponent for ModPlusOneDivFour<MOD> {
            const EXPONENT: U256 = MOD::MOD_PLUS_1_DIV_4;
        }
        let res = self.pow_fixed::<ModPlusOneDivFour<MOD>>();
        let res_square = res.square();
        (res, res_square.ct_eq(self))
    }

    #[inline(always)]
    const fn sqrt_p_mod_8_is_5(&self) -> (Self, CtChoice) {
        // Using recipe from RFC-8032 sqrt8k5.
        struct FixedExpMod3_8<MOD: MontyParams>(PhantomData<MOD>);
        impl<MOD: MontyParams> FixedExponent for FixedExpMod3_8<MOD> {
            const EXPONENT: U256 = MOD::MOD_3_8;
        }

        let tv1 = self.pow_fixed::<FixedExpMod3_8<MOD>>();
        let tv2 = tv1.mul(&MOD::SQRT_M1);
        let candidate = Self::ct_select(&tv2, &tv1, tv1.square().ct_eq(self));
        let candidate_squared = candidate.square();
        let sq_eq_self = candidate_squared.ct_eq(self);
        (candidate, sq_eq_self)
    }
}

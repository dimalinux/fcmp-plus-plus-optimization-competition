// TODO: Use `adt_const_params` once stabilized to make a `Residue` generic around a modulus rather than having to implement a ZST + trait
#[macro_export]
/// Implements a modulus with the given name, type, and value, in that specific order. Please `use crypto_bigint::traits::Encoding` to make this work.
/// For example, `impl_modulus!(MyModulus, U256, "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001");` implements a 256-bit modulus named `MyModulus`.
/// The modulus _must_ be odd, or this will panic.
macro_rules! impl_modulus {
    ($name:ident, $value:expr) => {
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct $name {}
        impl $crate::bigint::ResidueParams for $name {
            const MODULUS: U256 = {
                let res = <U256>::from_be_hex($value);

                // Check that the modulus is odd
                if res.as_limbs()[0].0 & 1 == 0 {
                    panic!("modulus must be odd");
                }

                res
            };
            const MOD_NEG_INV: $crate::bigint::Limb = $crate::bigint::Limb(
                $crate::bigint::Word::MIN.wrapping_sub(
                    Self::MODULUS
                        .inv_mod2k_vartime($crate::bigint::Word::BITS as usize)
                        .as_limbs()[0]
                        .0,
                ),
            );
            const R: U256 = $crate::bigint::Uint::MAX
                .const_rem(&Self::MODULUS)
                .0
                .wrapping_add(&$crate::bigint::Uint::ONE);
            const R2: U256 =
                $crate::bigint::Uint::const_rem_wide(Self::R.square_wide(), &Self::MODULUS).0;
            const R3: U256 = $crate::bigint::uint::modular::montgomery_reduction(
                &Self::R2.square_wide(),
                &Self::MODULUS,
                Self::MOD_NEG_INV,
            );
        }
    };
}

#[macro_export]
/// Creates a `Residue` with the given value for a specific modulus.
/// For example, `residue!(U256::from(105u64), MyModulus);` creates a `Residue` for 105 mod `MyModulus`.
/// The modulus _must_ be odd, or this will panic.
macro_rules! const_residue {
    ($variable:ident, $modulus:ident) => {
        $crate::bigint::Residue::<$modulus>::new(&$variable)
    };
}

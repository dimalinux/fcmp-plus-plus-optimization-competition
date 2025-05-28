use super::{MontyForm, MontyParams};
use crate::u256::{CtChoice, U256};

pub(crate) trait FixedExponent {
    /// The exponent used when taking the power of a field element.
    const EXPONENT: U256;
    const EXPONENT_NIBLES: [u8; 64] = Self::EXPONENT.as_be_nibbles();
}

impl<MOD: MontyParams> MontyForm<MOD> {
    #[must_use]
    pub(crate) const fn pow_fixed<EXP: FixedExponent>(self) -> Self {
        let mut table: [Self; 16] = [Self::ZERO; 16];
        table[0] = Self::ONE;
        table[1] = self;
        table[2] = Self::mul(&table[1], &self);
        table[3] = Self::mul(&table[2], &self);
        table[4] = Self::mul(&table[3], &self);
        table[5] = Self::mul(&table[4], &self);
        table[6] = Self::mul(&table[5], &self);
        table[7] = Self::mul(&table[6], &self);
        table[8] = Self::mul(&table[7], &self);
        table[9] = Self::mul(&table[8], &self);
        table[10] = Self::mul(&table[9], &self);
        table[11] = Self::mul(&table[10], &self);
        table[12] = Self::mul(&table[11], &self);
        table[13] = Self::mul(&table[12], &self);
        table[14] = Self::mul(&table[13], &self);
        table[15] = Self::mul(&table[14], &self);

        let mut res = Self::ONE;
        let mut i = 0;
        while i < 64 {
            let bits = EXP::EXPONENT_NIBLES[i] as u64;
            i += 1;

            res = Self::square(&res);
            res = Self::square(&res);
            res = Self::square(&res);
            res = Self::square(&res);

            let mut factor = table[0];
            factor = Self::ct_select(&factor, &table[1], CtChoice::from_u64_eq(bits, 1));
            factor = Self::ct_select(&factor, &table[2], CtChoice::from_u64_eq(bits, 2));
            factor = Self::ct_select(&factor, &table[3], CtChoice::from_u64_eq(bits, 3));
            factor = Self::ct_select(&factor, &table[4], CtChoice::from_u64_eq(bits, 4));
            factor = Self::ct_select(&factor, &table[5], CtChoice::from_u64_eq(bits, 5));
            factor = Self::ct_select(&factor, &table[6], CtChoice::from_u64_eq(bits, 6));
            factor = Self::ct_select(&factor, &table[7], CtChoice::from_u64_eq(bits, 7));
            factor = Self::ct_select(&factor, &table[8], CtChoice::from_u64_eq(bits, 8));
            factor = Self::ct_select(&factor, &table[9], CtChoice::from_u64_eq(bits, 9));
            factor = Self::ct_select(&factor, &table[10], CtChoice::from_u64_eq(bits, 10));
            factor = Self::ct_select(&factor, &table[11], CtChoice::from_u64_eq(bits, 11));
            factor = Self::ct_select(&factor, &table[12], CtChoice::from_u64_eq(bits, 12));
            factor = Self::ct_select(&factor, &table[13], CtChoice::from_u64_eq(bits, 13));
            factor = Self::ct_select(&factor, &table[14], CtChoice::from_u64_eq(bits, 14));
            factor = Self::ct_select(&factor, &table[15], CtChoice::from_u64_eq(bits, 15));
            res = Self::mul(&res, &factor);
        }

        res
    }

    /// Perform exponentiation.
    #[must_use]
    pub(crate) const fn pow(self, exponent: Self) -> Self {
        let mut table: [Self; 16] = [Self::ZERO; 16];
        table[0] = Self::ONE;
        table[1] = self;
        table[2] = Self::mul(&table[1], &self);
        table[3] = Self::mul(&table[2], &self);
        table[4] = Self::mul(&table[3], &self);
        table[5] = Self::mul(&table[4], &self);
        table[6] = Self::mul(&table[5], &self);
        table[7] = Self::mul(&table[6], &self);
        table[8] = Self::mul(&table[7], &self);
        table[9] = Self::mul(&table[8], &self);
        table[10] = Self::mul(&table[9], &self);
        table[11] = Self::mul(&table[10], &self);
        table[12] = Self::mul(&table[11], &self);
        table[13] = Self::mul(&table[12], &self);
        table[14] = Self::mul(&table[13], &self);
        table[15] = Self::mul(&table[14], &self);

        let mut res = Self::ONE;
        let nibbles = exponent.retrieve().as_be_nibbles();
        let mut i = 0;
        while i < 64 {
            let bits = nibbles[i] as u64;
            i += 1;

            res = Self::square(&res);
            res = Self::square(&res);
            res = Self::square(&res);
            res = Self::square(&res);

            let mut factor = table[0];
            factor = Self::ct_select(&factor, &table[1], CtChoice::from_u64_eq(bits, 1));
            factor = Self::ct_select(&factor, &table[2], CtChoice::from_u64_eq(bits, 2));
            factor = Self::ct_select(&factor, &table[3], CtChoice::from_u64_eq(bits, 3));
            factor = Self::ct_select(&factor, &table[4], CtChoice::from_u64_eq(bits, 4));
            factor = Self::ct_select(&factor, &table[5], CtChoice::from_u64_eq(bits, 5));
            factor = Self::ct_select(&factor, &table[6], CtChoice::from_u64_eq(bits, 6));
            factor = Self::ct_select(&factor, &table[7], CtChoice::from_u64_eq(bits, 7));
            factor = Self::ct_select(&factor, &table[8], CtChoice::from_u64_eq(bits, 8));
            factor = Self::ct_select(&factor, &table[9], CtChoice::from_u64_eq(bits, 9));
            factor = Self::ct_select(&factor, &table[10], CtChoice::from_u64_eq(bits, 10));
            factor = Self::ct_select(&factor, &table[11], CtChoice::from_u64_eq(bits, 11));
            factor = Self::ct_select(&factor, &table[12], CtChoice::from_u64_eq(bits, 12));
            factor = Self::ct_select(&factor, &table[13], CtChoice::from_u64_eq(bits, 13));
            factor = Self::ct_select(&factor, &table[14], CtChoice::from_u64_eq(bits, 14));
            factor = Self::ct_select(&factor, &table[15], CtChoice::from_u64_eq(bits, 15));
            res = Self::mul(&res, &factor);
        }

        res
    }
}

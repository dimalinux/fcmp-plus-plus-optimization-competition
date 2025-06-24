use core::{fmt::Debug, marker::PhantomData};

use rand_core::RngCore;
use zeroize::Zeroize;

use crate::u256::{CtChoice, MontyForm, MontyParams, U256};

pub(super) trait PointParams<MOD: MontyParams>:
    Copy + Debug + Default + Eq + Send + Sync + 'static
{
    const B: MontyForm<MOD>;
    /// Generator point X coordinate
    const G_X: MontyForm<MOD>;
    /// Generator point Y coordinate
    const G_Y: MontyForm<MOD>;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Zeroize)]
pub(super) struct Point<MOD: MontyParams, P: PointParams<MOD>> {
    pub(super) x: MontyForm<MOD>,
    pub(super) y: MontyForm<MOD>,
    pub(super) z: MontyForm<MOD>,
    phantom_data: PhantomData<P>,
}

impl<MOD: MontyParams, P: PointParams<MOD>> Point<MOD, P> {
    const A: MontyForm<MOD> = MontyForm::new(&U256::from_u64(3)).neg();
    const B3: MontyForm<MOD> = P::B.add(&P::B).add(&P::B);
    pub(super) const G: Self = Self::new(P::G_X, P::G_Y, MontyForm::ONE);
    pub(super) const IDENTITY: Self = Self::new(MontyForm::ZERO, MontyForm::ONE, MontyForm::ZERO);

    pub(super) const fn new(x: MontyForm<MOD>, y: MontyForm<MOD>, z: MontyForm<MOD>) -> Self {
        Self {
            x,
            y,
            z,
            phantom_data: PhantomData,
        }
    }

    pub(super) const fn ct_select(a: &Self, b: &Self, c: CtChoice) -> Self {
        Self::new(
            MontyForm::ct_select(&a.x, &b.x, c),
            MontyForm::ct_select(&a.y, &b.y, c),
            MontyForm::ct_select(&a.z, &b.z, c),
        )
    }

    pub(super) const fn ct_eq(&self, other: &Self) -> CtChoice {
        let x1 = self.x.mul(&other.z);
        let x2 = other.x.mul(&self.z);
        let y1 = self.y.mul(&other.z);
        let y2 = other.y.mul(&self.z);
        let both_x_zero = self.x.ct_is_zero().and(other.x.ct_is_zero());
        let x_and_y_eq = x1.ct_eq(&x2).and(y1.ct_eq(&y2));
        both_x_zero.or(x_and_y_eq)
    }

    #[allow(non_snake_case)]
    pub(super) const fn add(self, other: &Self) -> Self {
        let X1 = &self.x;
        let Y1 = &self.y;
        let Z1 = &self.z;
        let X2 = &other.x;
        let Y2 = &other.y;
        let Z2 = &other.z;

        let t0 = X1.mul(X2);
        let t1 = Y1.mul(Y2);
        let t2 = Z1.mul(Z2);
        let t3 = X1.add(Y1).mul(&X2.add(Y2));
        let t4 = t0.add(&t1);
        let t3 = t3.sub(&t4);
        let t4 = X1.add(Z1);
        let t5 = X2.add(Z2);
        let t4 = t4.mul(&t5);
        let t5 = t0.add(&t2);
        let t4 = t4.sub(&t5);
        let t5 = Y1.add(Z1);
        let X3 = Y2.add(Z2);
        let t5 = t5.mul(&X3);
        let X3 = t1.add(&t2);
        let t5 = t5.sub(&X3);
        let Z3 = Self::A.mul(&t4);
        let X3 = Self::B3.mul(&t2);
        let Z3 = X3.add(&Z3);
        let X3 = t1.sub(&Z3);
        let Z3 = t1.add(&Z3);
        let Y3 = X3.mul(&Z3);
        let t1 = t0.add(&t0);
        let t1 = t1.add(&t0);
        let t2 = Self::A.mul(&t2);
        let t4 = Self::B3.mul(&t4);
        let t1 = t1.add(&t2);
        let t2 = t0.sub(&t2);
        let t2 = Self::A.mul(&t2);
        let t4 = t4.add(&t2);
        let t0 = t1.mul(&t4);
        let Y3 = Y3.add(&t0);
        let t0 = t5.mul(&t4);
        let X3 = t3.mul(&X3);
        let X3 = X3.sub(&t0);
        let t0 = t3.mul(&t1);
        let Z3 = t5.mul(&Z3);
        let Z3 = Z3.add(&t0);

        Self::new(X3, Y3, Z3)
    }

    pub(super) const fn sub(self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    #[allow(non_snake_case)]
    pub(super) const fn double(&self) -> Self {
        let X1 = self.x;
        let Y1 = self.y;
        let Z1 = self.z;
        let w = X1.sub(&Z1).mul(&X1.add(&Z1));
        let w = w.add(&w).add(&w);
        let s = Y1.mul(&Z1).double();
        let ss = s.square();
        let sss = s.mul(&ss);
        let R = Y1.mul(&s);
        let RR = R.square();
        let B_ = X1.mul(&R).double();
        let h = w.square().sub(&B_.double());
        let X3 = h.mul(&s);
        let Y3 = w.mul(&B_.sub(&h)).sub(&RR.double());
        let Z3 = sss;

        let is_identity = self.x.ct_is_zero();

        Self::ct_select(&Self::new(X3, Y3, Z3), &Self::IDENTITY, is_identity)
    }

    pub(super) const fn neg(&self) -> Self {
        Self::new(self.x, self.y.neg(), self.z)
    }

    pub(super) const fn is_identity(&self) -> CtChoice {
        self.x.ct_is_zero()
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(super) const fn to_bytes(self) -> [u8; 32] {
        let (z, c) = self.z.invert();
        if !c.is_true_vartime() {
            return [0; 32];
        }

        let x = self.x.mul(&z);
        let y = self.y.mul(&z);
        let mut bytes = x.retrieve().to_le_bytes();
        let y_lsb = y.retrieve().least_significant_bit();
        let x_is_zero = x.ct_is_zero();
        let y_sign = x_is_zero.select(y_lsb, 0);
        bytes[31] |= (y_sign << 7) as u8;

        bytes
    }

    pub(super) const fn mul<SMOD: MontyParams>(self, scalar: &MontyForm<SMOD>) -> Self {
        let mut table = [Self::IDENTITY; 16];
        table[1] = self;
        let mut i = 2;
        while i < 16 {
            table[i] = self.add(&table[i - 1]);
            i += 1;
        }

        let mut res = Self::IDENTITY;
        let scalar = scalar.retrieve();
        let mut bits: u8;
        let mut i = 0;
        while i < 64 {
            bits = scalar.nibble_be(i);

            if i > 0 {
                res = res.double();
                res = res.double();
                res = res.double();
                res = res.double();
            }

            i += 1;

            let mut term = table[0];
            let mut j: usize = 1;
            while j < 16 {
                let c = CtChoice::from_u64_eq(bits as u64, j as u64);
                term = Self::ct_select(&term, &table[j], c);
                j += 1;
            }
            res = res.add(&term);
        }
        // TODO: Zeroize crate isn't const fn friendly.
        // Do we need to zeroize scalar and bits? The caller if the
        // scalar on the operator overloading supposedly uses a copy
        // (even though the compiler generated code unlikely copies).
        // Force inline the operator overloading?
        res
    }

    pub(super) const fn recover_y(x: MontyForm<MOD>) -> (MontyForm<MOD>, CtChoice) {
        // ((x.square() * x) - x - x - x + B).sqrt()
        let mut v = x.square();
        v = v.mul(&x);
        v = v.sub(&x);
        v = v.sub(&x);
        v = v.sub(&x);
        v = v.add(&P::B);

        v.sqrt()
    }

    pub(super) const fn from_bytes(bytes: &[u8; 32]) -> (Self, CtChoice) {
        let sign_bit = (bytes[31] >> 7) as u64;
        let mut bytes = *bytes;
        bytes[31] &= !(1 << 7);

        let (x, less_than_modulus) = MontyForm::from_le_bytes(bytes);
        let (mut y, sq_rt_exists) = Self::recover_y(x);
        let even_odd_bit = y.retrieve().least_significant_bit();
        let needs_negation = CtChoice::from_lsb(sign_bit ^ even_odd_bit);
        y = y.ct_neg(needs_negation);

        let is_identity = x.ct_is_zero();
        y = MontyForm::ct_select(&y, &MontyForm::ONE, is_identity);

        let pt = Self::new(x, y, MontyForm::ONE);
        let sign_bit = CtChoice::from_lsb(sign_bit);
        let not_negative_zero = is_identity.and(sign_bit).not();

        let is_valid = less_than_modulus
            .and(sq_rt_exists.or(is_identity))
            .and(not_negative_zero);

        (pt, is_valid)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(super) fn random(mut rng: impl RngCore) -> Self {
        loop {
            let mut bytes = MontyForm::<MOD>::random(&mut rng).to_le_bytes();
            bytes[31] |= ((rng.next_u32() & 0x1) << 7) as u8;
            let (pt, c) = Self::from_bytes(&bytes);
            if c.is_true_vartime() {
                return pt;
            }
        }
    }
}

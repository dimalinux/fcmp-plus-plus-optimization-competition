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
    const A: MontyForm<MOD> = MontyForm::neg(&MontyForm::new(&U256::from_u64(3)));
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
        let x1 = MontyForm::mul(&self.x, &other.z);
        let x2 = MontyForm::mul(&other.x, &self.z);
        let y1 = MontyForm::mul(&self.y, &other.z);
        let y2 = MontyForm::mul(&other.y, &self.z);
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

        let t0 = MontyForm::mul(X1, X2);
        let t1 = MontyForm::mul(Y1, Y2);
        let t2 = MontyForm::mul(Z1, Z2);
        let t3 = MontyForm::mul(&MontyForm::add(X1, Y1), &MontyForm::add(X2, Y2));
        let t4 = MontyForm::add(&t0, &t1);
        let t3 = MontyForm::sub(&t3, &t4);
        let t4 = MontyForm::add(X1, Z1);
        let t5 = MontyForm::add(X2, Z2);
        let t4 = MontyForm::mul(&t4, &t5);
        let t5 = MontyForm::add(&t0, &t2);
        let t4 = MontyForm::sub(&t4, &t5);
        let t5 = MontyForm::add(Y1, Z1);
        let X3 = MontyForm::add(Y2, Z2);
        let t5 = MontyForm::mul(&t5, &X3);
        let X3 = MontyForm::add(&t1, &t2);
        let t5 = MontyForm::sub(&t5, &X3);
        let Z3 = MontyForm::mul(&Self::A, &t4);
        let X3 = MontyForm::mul(&Self::B3, &t2);
        let Z3 = MontyForm::add(&X3, &Z3);
        let X3 = MontyForm::sub(&t1, &Z3);
        let Z3 = MontyForm::add(&t1, &Z3);
        let Y3 = MontyForm::mul(&X3, &Z3);
        let t1 = MontyForm::add(&t0, &t0);
        let t1 = MontyForm::add(&t1, &t0);
        let t2 = MontyForm::mul(&Self::A, &t2);
        let t4 = MontyForm::mul(&Self::B3, &t4);
        let t1 = MontyForm::add(&t1, &t2);
        let t2 = MontyForm::sub(&t0, &t2);
        let t2 = MontyForm::mul(&Self::A, &t2);
        let t4 = MontyForm::add(&t4, &t2);
        let t0 = MontyForm::mul(&t1, &t4);
        let Y3 = MontyForm::add(&Y3, &t0);
        let t0 = MontyForm::mul(&t5, &t4);
        let X3 = MontyForm::mul(&t3, &X3);
        let X3 = MontyForm::sub(&X3, &t0);
        let t0 = MontyForm::mul(&t3, &t1);
        let Z3 = MontyForm::mul(&t5, &Z3);
        let Z3 = MontyForm::add(&Z3, &t0);

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
        let w = MontyForm::mul(&MontyForm::sub(&X1, &Z1), &MontyForm::add(&X1, &Z1));
        let w = MontyForm::add(&MontyForm::add(&w, &w), &w);
        let s = MontyForm::double(&MontyForm::mul(&Y1, &Z1));
        let ss = MontyForm::square(&s);
        let sss = MontyForm::mul(&s, &ss);
        let R = MontyForm::mul(&Y1, &s);
        let RR = R.square();
        let B_ = MontyForm::mul(&X1, &R).double();
        let h = MontyForm::sub(&w.square(), &B_.double());
        let X3 = MontyForm::mul(&h, &s);
        let Y3 = MontyForm::sub(
            &MontyForm::mul(&w, &(MontyForm::sub(&B_, &h))),
            &RR.double(),
        );
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

        let x = MontyForm::mul(&self.x, &z);
        let y = MontyForm::mul(&self.y, &z);
        let mut bytes = x.retrieve().to_le_bytes();
        let y_lsb = y.retrieve().least_significant_bit();
        let x_is_zero = x.ct_is_zero();
        let y_sign = x_is_zero.select(y_lsb, 0);
        bytes[31] |= (y_sign << 7) as u8;

        bytes
    }

    pub(super) const fn mul<SMOD: MontyParams>(self, scalar: MontyForm<SMOD>) -> Self {
        let mut table = [Self::IDENTITY; 16];
        table[1] = self;
        let mut i = 2;
        while i < 16 {
            table[i] = Self::add(self, &table[i - 1]);
            i += 1;
        }

        let mut res = Self::IDENTITY;
        let nibbles = scalar.retrieve().as_be_nibbles();
        let mut i = 0;
        #[allow(unused_assignments)]
        while i < 64 {
            let mut bits = nibbles[i];

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
            res = Self::add(res, &term);
            bits = 0;
        }
        // TODO: How to handle this in a const function?
        //nibbles.zeroize();
        //other.zeroize();
        res
    }

    pub(super) const fn recover_y(x: MontyForm<MOD>) -> (MontyForm<MOD>, CtChoice) {
        // ((x.square() * x) - x - x - x + B).sqrt()
        let mut v = MontyForm::square(&x);
        v = MontyForm::mul(&v, &x);
        v = MontyForm::sub(&v, &x);
        v = MontyForm::sub(&v, &x);
        v = MontyForm::sub(&v, &x);
        v = MontyForm::add(&v, &P::B);

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

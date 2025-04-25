use crate::{Concat, ConcatMixed, Limb, Uint};

impl<T> Concat for T
where
    T: ConcatMixed<T>,
{
    type Output = Self::MixedOutput;
}

/// Concatenate the two values, with `lo` as least significant and `hi`
/// as the most significant.
#[inline]
pub(crate) const fn concat_mixed<const L: usize, const H: usize, const O: usize>(
    lo: &Uint<L>,
    hi: &Uint<H>,
) -> Uint<O> {
    let top = L + H;
    let top = if top < O { top } else { O };
    let mut limbs = [Limb::ZERO; O];
    let mut i = 0;

    while i < top {
        if i < L {
            limbs[i] = lo.limbs[i];
        } else {
            limbs[i] = hi.limbs[i - L];
        }
        i += 1;
    }

    Uint { limbs }
}

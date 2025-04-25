use super::Uint;

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Construct a `Uint<T>` from the unsigned integer value,
    /// truncating the upper bits if the value is too large to be
    /// represented.
    #[inline(always)]
    pub const fn resize<const T: usize>(&self) -> Uint<T> {
        let mut res = Uint::ZERO;
        let mut i = 0;
        let dim = if T < LIMBS { T } else { LIMBS };
        while i < dim {
            res.limbs[i] = self.limbs[i];
            i += 1;
        }
        res
    }
}

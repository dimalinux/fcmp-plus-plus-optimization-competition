// Use black_box when possible
#[rustversion::since(1.66)]
use core::hint::black_box;

use zeroize::Zeroize;
#[rustversion::before(1.66)]
fn black_box<T>(val: T) -> T {
    val
}

pub(crate) fn u8_from_bool(bit_ref: &mut bool) -> u8 {
    let bit_ref = black_box(bit_ref);

    let mut bit = black_box(*bit_ref);
    let res = black_box(u8::from(bit));
    bit.zeroize();
    debug_assert!((res | 1) == 1);

    bit_ref.zeroize();
    res
}

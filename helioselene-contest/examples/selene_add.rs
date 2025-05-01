#![allow(non_snake_case)]

use helioselene::{
    group::{Group, GroupEncoding},
    SelenePoint as SelenePointRef,
};
use helioselene_contest_src::SelenePoint;
use rand_core::OsRng;

pub fn gen_random_selene_point() -> SelenePoint {
    let A_ref = SelenePointRef::random(&mut OsRng);
    let A = SelenePoint::from_bytes(&A_ref.to_bytes()).expect("Failed to read selene point");
    assert_eq!(A.to_bytes(), A_ref.to_bytes());
    A
}

fn main() {
    // Do all the initialization
    let pt1: SelenePoint = gen_random_selene_point();
    let pt2: SelenePoint = gen_random_selene_point();

    for _ in 0..2_000_000 {
        let _ = core::hint::black_box(pt1 + pt2);
    }
}

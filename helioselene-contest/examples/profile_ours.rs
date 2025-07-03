#![allow(non_snake_case)]

use helioselene::group::{Group, GroupEncoding};
use helioselene_contest_src::{
    group::ff::Field, Field25519, HeliosPoint, HelioseleneField, SelenePoint,
};
use rand_core::OsRng;

fn main() {
    // Do all the initialization
    let A_S = SelenePoint::random(&mut OsRng);
    let B_S = SelenePoint::random(&mut OsRng);

    let A_H = HeliosPoint::random(&mut OsRng);
    let B_H = HeliosPoint::random(&mut OsRng);
    let a_h = HelioseleneField::random(&mut OsRng);
    let b_h = HelioseleneField::random(&mut OsRng);
    let a_s = Field25519::random(&mut OsRng);
    let A_S_bytes = A_S.to_bytes();
    let A_H_bytes = A_H.to_bytes();

    // SelenePointAdd
    for _ in 0..2_000_000 {
        let _ = core::hint::black_box(A_S + B_S);
    }

    // HeliosPointAdd
    for _ in 0..2_000_000 {
        let _ = core::hint::black_box(A_H + B_H);
    }

    // HelioseleneMul
    for _ in 0..50_000_000 {
        let _ = core::hint::black_box(a_h * b_h);
    }

    // HelioseleneInvert
    for _ in 0..200_000 {
        let _ = core::hint::black_box(a_h.invert());
    }

    // SelenePointDecompression
    for _ in 0..100_000 {
        let _ = core::hint::black_box(SelenePoint::from_bytes(&A_S_bytes));
    }

    // HeliosPointDecompression
    for _ in 0..100_000 {
        let _ = core::hint::black_box(HeliosPoint::from_bytes(&A_H_bytes));
    }

    // HelioseleneAdd
    for _ in 0..200_000_000 {
        let _ = core::hint::black_box(a_h + b_h);
    }

    // HelioseleneSub
    for _ in 0..200_000_000 {
        let _ = core::hint::black_box(a_h - b_h);
    }

    // SelenePointMul
    for _ in 0..10_000 {
        let _ = core::hint::black_box(A_S * a_s);
    }

    // HeliosPointMul
    for _ in 0..10_000 {
        let _ = core::hint::black_box(A_H * a_h);
    }
}

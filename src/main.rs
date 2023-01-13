#![feature(portable_simd)]

use std::simd::f32x4;

fn main() {
    let x = f32x4::splat(1.0);
    let y = f32x4::from_array([4.0, 3.0, 2.0, 1.0]);
    let z = x + y;

    println!("{z:?}");
}

#![feature(portable_simd)]

use std::simd::i16x8;
use std::simd::SimdPartialOrd;

pub mod data;

pub mod prelude {
    pub use crate::data::ARRAY;
    pub use crate::problem_1;
}

pub fn problem_1(array: &[i16]) -> i16 {
    let zero = i16x8::splat(0);

    let mut positives = i16x8::splat(0);
    let mut negatives = i16x8::splat(0);

    let mut i = 0;
    while i + 7 < array.len() {
        let a = i16x8::from_slice(&array[i..i + 8]);
        positives += a.simd_gt(zero).to_int();
        negatives += a.simd_lt(zero).to_int();
        i += 8;
    }

    let n_positive = -positives.to_array().into_iter().sum::<i16>();
    let n_negative = -negatives.to_array().into_iter().sum::<i16>();

    std::cmp::max(n_positive, n_negative)
}

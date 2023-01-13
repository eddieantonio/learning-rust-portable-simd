#![feature(portable_simd)]

use std::simd::i16x8;
use std::simd::{SimdInt, SimdPartialOrd};

pub mod data;

pub mod prelude {
    pub use crate::data::ARRAY;
    pub use crate::problem_1;
}

/// Solves
pub fn problem_1(array: &[i16]) -> i16 {
    let zero = i16x8::splat(0);

    let mut positives = i16x8::splat(0);
    let mut negatives = i16x8::splat(0);

    let (prefix, middle, suffix) = array.as_simd();

    for chunk in middle {
        positives += chunk.simd_gt(zero).to_int();
        negatives += chunk.simd_lt(zero).to_int();
    }

    let mut n_positive = -positives.reduce_sum();
    let mut n_negative = -negatives.reduce_sum();

    for n in prefix.iter().copied() {
        n_positive += (n > 0) as i16;
        n_negative += (n < 0) as i16;
    }

    for n in suffix.iter().copied() {
        n_positive += (n > 0) as i16;
        n_negative += (n < 0) as i16;
    }

    std::cmp::max(n_positive, n_negative)
}

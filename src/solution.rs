use std::simd::i16x8;
use std::simd::{SimdInt, SimdPartialOrd};

#[derive(Copy, Clone, Debug)]
struct Counts(i16, i16);

/// Solves leetcode contest 327, problem 1.
///
/// "Maximum Count of Positive Integer and Negative Integer"
///
/// https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer/
///
/// I wrote this code to teach myself how to use Rust's portable_simd. I chose i16x8 because:
///  * the numbers satisfy this property: -2000 <= `nums[i]` <= 2000.
///     (this fits in within -32768 <= i16 <= 32767)
///  * I'm on an M1 Macbook, which has AArch64 Neon (128-bit SIMD) support.
pub fn problem_1(nums: &[i16]) -> i16 {
    let Counts(n_positive, n_negative) = count_pos_neg_vectorized(nums);
    std::cmp::max(n_positive, n_negative)
}

fn count_pos_neg_vectorized(nums: &[i16]) -> Counts {
    let (prefix, middle, suffix) = nums.as_simd();

    // To ensure memory access are in order, access the prefix first:
    let prefix_counts = count_pos_neg_non_vectorized(prefix);

    // Now we can count positives and negatives in the middle, 8 values at a time!
    let zero = i16x8::splat(0);
    let mut positives = i16x8::splat(0);
    let mut negatives = i16x8::splat(0);
    for chunk in middle {
        // Because Mask::to_int() returns -1 for each true value, we have to **subtract**
        // that -1 from the accumulator in order to add 1. That is:
        //
        //  current == previous - true
        //          == previous - -1
        //          == previous + 1
        positives -= chunk.simd_gt(zero).to_int();
        negatives -= chunk.simd_lt(zero).to_int();
    }
    let middle_counts = (positives.reduce_sum(), negatives.reduce_sum()).into();

    // Finally, the suffix:
    let suffix_counts = count_pos_neg_non_vectorized(suffix);

    prefix_counts + middle_counts + suffix_counts
}

#[inline]
fn count_pos_neg_non_vectorized(array: &[i16]) -> Counts {
    array
        .iter()
        .copied()
        .fold((0, 0), |(n_positive, n_negative), n| {
            (n_positive + (n > 0) as i16, n_negative + (n < 0) as i16)
        })
        .into()
}

impl From<(i16, i16)> for Counts {
    fn from((a, b): (i16, i16)) -> Self {
        Counts(a, b)
    }
}

impl std::ops::Add for Counts {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

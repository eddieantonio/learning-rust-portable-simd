//! A safe wrapper for the C implementation.
//!
//!
use super::_c_implementation;

pub fn problem_1(nums: &[i16]) -> usize {
    unsafe { _c_implementation::problem_1_for(nums.as_ptr(), nums.len()) }
}

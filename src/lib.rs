#![feature(portable_simd)]

pub mod data;
pub mod implementations;

pub mod prelude {
    /// Default test data.
    pub use crate::data::ARRAY;
    /// Default implementation is the SIMD version.
    pub use crate::implementations::simd::problem_1;
}

#[cfg(test)]
mod tests {
    use super::{data, implementations};

    #[test]
    fn test_simd() {
        assert_eq!(data::ANSWER, implementations::simd::problem_1(&data::ARRAY));
    }
}

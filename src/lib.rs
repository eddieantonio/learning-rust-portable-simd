#![feature(portable_simd)]
#![feature(test)]

#[cfg(test)]
extern crate test;

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
    use super::data;
    use test::Bencher;

    #[test]
    fn test_binary() {
        use crate::implementations::binary::problem_1;
        assert_eq!(data::ANSWER, problem_1(&data::ARRAY));
    }

    #[test]
    fn test_c() {
        use crate::implementations::c::problem_1;
        assert_eq!(data::ANSWER, problem_1(&data::ARRAY));
    }

    #[test]
    fn test_fold() {
        use crate::implementations::fold::problem_1;
        assert_eq!(data::ANSWER, problem_1(&data::ARRAY));
    }

    #[test]
    fn test_simd() {
        use crate::implementations::simd::problem_1;
        assert_eq!(data::ANSWER, problem_1(&data::ARRAY));
    }

    #[bench]
    fn bench_binary(b: &mut Bencher) {
        use crate::implementations::binary::problem_1;
        b.iter(|| problem_1(&data::ARRAY));
    }

    #[bench]
    fn bench_c(b: &mut Bencher) {
        use crate::implementations::c::problem_1;
        b.iter(|| problem_1(&data::ARRAY));
    }

    #[bench]
    fn bench_fold(b: &mut Bencher) {
        use crate::implementations::fold::problem_1;
        b.iter(|| problem_1(&data::ARRAY));
    }

    #[bench]
    fn bench_simd(b: &mut Bencher) {
        use crate::implementations::simd::problem_1;
        b.iter(|| problem_1(&data::ARRAY));
    }
}

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
    use test::Bencher;

    macro_rules! impl_test {
        ($mod: ident) => {
            #[test]
            fn $mod() {
                use super::{data, implementations};
                assert_eq!(data::ANSWER, implementations::$mod::problem_1(&data::ARRAY));
            }
        };
    }

    impl_test!(simd);
    impl_test!(fold);
    impl_test!(c);

    use crate::data;

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

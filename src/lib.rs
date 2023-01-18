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
}

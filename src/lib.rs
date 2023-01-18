#![feature(portable_simd)]

pub mod data;
pub mod implementations;

pub mod prelude {
    /// Default test data.
    pub use crate::data::ARRAY;
    /// Default implementation is the SIMD version.
    pub use crate::implementations::simd::problem_1;
}

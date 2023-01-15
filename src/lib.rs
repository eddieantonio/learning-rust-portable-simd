#![feature(portable_simd)]

pub mod data;
mod solution;

pub mod prelude {
    pub use crate::data::ARRAY;
    pub use crate::solution::problem_1;
}

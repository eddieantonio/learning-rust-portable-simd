#[link(name = "problem1")]
extern "C" {
    pub fn problem_1(array: *const i16, n: usize) -> usize;
}

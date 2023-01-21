#[link(name = "problem1")]
extern "C" {
    pub fn problem_1_binary(array: *const i16, n: usize) -> usize;
    pub fn problem_1_for(array: *const i16, n: usize) -> usize;
}

use std::process::Command;

fn main() {
    create_sample_data();
    compile_c_library();
}

fn compile_c_library() {
    cc::Build::new()
        .file("./src/implementations/c-implementation.c")
        .compile("libproblem1.a");
}

fn create_sample_data() {
    Command::new("python")
        .args(["./create-array.py", "2000"])
        .status()
        .unwrap();
}

use std::process::Command;

fn main() {
    create_sample_data();
    compile_c_library();
}

fn compile_c_library() {
    let source = "src/implementations/c-implementation.c";
    cc::Build::new().file(source).compile("libproblem1.a");
    println!("cargo:rerun-if-changed={source}");
}

fn create_sample_data() {
    Command::new("python")
        .args(["./create-array.py", "2000"])
        .status()
        .unwrap();
}

use std::process::Command;

fn main() {
    Command::new("python").args(&["./create-array.py", "2000"])
        .status().unwrap();
}

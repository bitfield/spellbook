use std::fs;

fn main() {
    if fs::metadata("Cargo.toml").is_ok() {
        println!("Yep, `Cargo.toml` exists.");
    }

    if fs::metadata("Cargo.toml").is_ok_and(|meta| meta.is_file()) {
        println!("Yep, `Cargo.toml` is a file.");
    }
}

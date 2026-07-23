#[expect(clippy::absolute_paths, reason = "explicitness")]
fn main() {
    let mut x = 42;
    let y = std::mem::replace(&mut x, 69);
    println!("y: {y}, x: {x}");
    // y: 42, x: 69
}

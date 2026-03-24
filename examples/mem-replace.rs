fn main() {
    let mut x = 42;
    let y = std::mem::replace(&mut x, 69);
    println!("{y}");
    // 42
}

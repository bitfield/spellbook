fn main() {
    use std::mem;
    let mut x = 42;
    let y = mem::replace(&mut x, 69);
    println!("{y}");
    // 42
}

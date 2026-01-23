fn main() {
    let value = 42;
    println!("{}", value.clamp(0, 10));
    // 10
    println!("{}", value.clamp(-50, 50));
    // 42
    println!("{}", value.clamp(50, 100));
    // 50
}

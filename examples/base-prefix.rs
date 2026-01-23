fn main() {
    // Normal `Debug` format:
    println!("{:b}", 42);
    // 101010
    println!("{:o}", 42);
    // 52
    println!("{:x}", 42);
    // 2a
    println!("{:X}", 42);
    // 2A

    // Alternate `Debug` format:
    println!("{:#b}", 42);
    // 0b101010
    println!("{:#o}", 42);
    // 0o52
    println!("{:#x}", 42);
    // 0x2a
    println!("{:#X}", 42);
    // 0x2A
}

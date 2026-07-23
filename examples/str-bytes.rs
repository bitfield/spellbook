fn main() {
    for byte in "Why nøt trei oür møøse burger?".as_bytes() {
        print!("{byte:02X} ");
    }
    // 57 68 79 20 6E C3 B8 74 20 74 72 65 69 20 6F C3 BC 72
    // 20 6D C3 B8 C3 B8 73 65 20 62 75 72 67 65 72 3F
}

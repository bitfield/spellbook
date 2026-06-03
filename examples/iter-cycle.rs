fn main() {
    for num in (0..3).cycle().take(12) {
        print!("{num} ");
    }
    // 0 1 2 0 1 2 0 1 2 0 1 2
}

fn main() {
    for num in (0..10).map(|n: usize| n.strict_mul(2)) {
        print!("{num} ");
    }
    // 0 2 4 6 8 10 12 14 16 18
}

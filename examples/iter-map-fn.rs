fn main() {
    for num in [1, -2, 3, -4, 5].into_iter().map(i32::abs) {
        print!("{num} ");
    }
    // 1 2 3 4 5
}

#[expect(clippy::for_unbounded_range, reason = "example")]
fn main() {
    for num in 0.. {
        print!("{num} ");
    }
    // 0 1 2 3 4 5...
}

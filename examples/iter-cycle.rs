fn main() {
    let nums = 0..3;
    for num in nums.cycle().take(12) {
        print!("{num} ");
    }
    // 0 1 2 0 1 2 0 1 2 0 1 2
}

fn main() {
    let nums = 0..3;
    for n in nums.cycle().take(12) {
        print!("{n} ");
    }
    // 0 1 2 0 1 2 0 1 2 0 1 2
}

fn main() {
    let nums = [1, -1, 1, -1, 1];
    let abs_nums = nums.into_iter().map(i32::abs);
    for a in abs_nums {
        print!("{a} ");
    }
    // 1 1 1 1 1
}

fn main() {
    let nums: [i32; _] = [1, 2, 3];
    let sum = nums
        .into_iter()
        .reduce(|sum, n| sum.strict_add(n))
        .unwrap_or_default();
    println!("{sum}");
    // 6
}

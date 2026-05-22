#[expect(clippy::iter_cloned_collect, reason = "example")]
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];
    let copies: Vec<i32> = nums.iter().copied().collect();
    println!("{copies:?}");
    // [1, 2, 3]
}

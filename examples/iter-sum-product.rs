#[expect(clippy::default_numeric_fallback, reason = "for readability")]
fn main() {
    let sum: i32 = [1, 2, 3].into_iter().sum();
    println!("{sum}");
    // 6
    // 
    let product: i32 = [4, 5, 6].into_iter().product();
    println!("{product}");
    // 120
}

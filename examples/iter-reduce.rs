fn main() {
    let sum = [1i32, 2, 3]
        .into_iter()
        .reduce(|sum, n| sum.strict_add(n))
        .unwrap_or_default();
    println!("{sum}");
    // 6
}

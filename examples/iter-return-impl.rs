fn main() {
    for p in evens() {
        println!("{p}");
    }
    // 0, 2, 4, 6, 8, ...
}

fn evens() -> impl Iterator<Item = usize> {
    (0..).map(|n: usize| n.strict_mul(2))
}

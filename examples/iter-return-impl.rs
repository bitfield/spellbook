fn main() {
    for even in evens() {
        println!("{even}");
    }
    // 0, 2, 4, 6, 8, ...
}

fn evens() -> impl Iterator<Item = usize> {
    (0..).map(|num: usize| num.strict_mul(2))
}

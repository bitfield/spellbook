fn main() {
    let evens = (0..10).map(|n: usize| n.strict_mul(2));
    for n in evens {
        print!("{n} ");
    }
    // 0 2 4 6 8 10 12 14 16 18
}

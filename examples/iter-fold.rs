fn main() {
    let mut sum = 0_i32;
    for n in [1, 2, 3] {
        sum = sum.strict_add(n);
    }
    println!("{sum}");
    // 6

    #[expect(
        clippy::shadow_unrelated,
        reason = "showing different approach"
    )]
    let sum = [1, 2, 3]
        .into_iter()
        .fold(0_i32, |sum, n| sum.strict_add(n));
    println!("{sum}");
    // 6

    let menu = [
        "sausage and chips",
        "steak and chips",
        "bacon and eggs",
        "sausage, beans, and chips",
    ];
    let (chips, soss): (usize, usize) =
        menu.iter().fold((0, 0), |(mut chips, mut soss), dish| {
            if dish.contains("chips") {
                chips = chips.strict_add(1);
            }
            if dish.contains("sausage") {
                soss = soss.strict_add(1);
            }
            (chips, soss)
        });
    println!("{chips} dishes with chips, {soss} with sausage");
    // 3 dishes with chips, 2 dishes with sausage
}

use std::iter;

use rand::seq::IndexedRandom as _;

#[expect(clippy::expect_used, reason = "called on static")]
fn main() {
    let pot_luck = iter::repeat_with(|| {
        ["fajitas", "burger", "burrito"]
            .choose(&mut rand::rng())
            .expect("no food!")
    });
    for dish in pot_luck.take(3) {
        println!("{dish}");
    }
    // fajitas
    // burger
    // fajitas
}

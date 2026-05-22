use rand::seq::IndexedRandom;

use std::iter::repeat_with;

fn main() {
    let pot_luck = repeat_with(|| {
        ["fajitas", "burger", "burrito"]
            .choose(&mut rand::rng())
            .expect("no food!")
    });
    for dish in pot_luck.take(3) {
        println!("{dish}");
    }
}

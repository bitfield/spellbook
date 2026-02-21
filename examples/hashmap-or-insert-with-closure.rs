use jiff::Zoned;

use std::collections::HashMap;

fn main() {
    let mut birds = HashMap::new();
    birds
        .entry("blackbird")
        .or_insert_with(|| Zoned::now().date());
    for (bird, sighting) in birds {
        println!("{bird}: {sighting}");
    }
    // blackbird: 2026-02-20
}

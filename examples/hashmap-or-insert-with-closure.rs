fn main() {
    use std::collections::HashMap;

    let mut birds = HashMap::new();
    birds
        .entry("blackbird")
        .or_insert_with(|| jiff::Zoned::now().date());
    for (bird, sighting) in birds {
        println!("{bird}: {sighting}");
    }
    // blackbird: 2026-02-20
}

use std::iter::chain;

fn main() {
    let entrées = ["steak", "lobster", "pasta"];
    let desserts = ["trifle", "strawberries", "death by chocolate"];
    for dish in chain(entrées, desserts) {
        println!("{dish}");
    }
    // steak
    // lobster
    // pasta
    // trifle
    // strawberries
    // death by chocolate
}

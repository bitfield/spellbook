fn main() {
    for food in some_breakfast() {
        println!("Mmm, {food:?}.");
    }
    // Mmm, Some("bacon").
    // Mmm, Some("sausages").
    // Mmm, Some("hash browns").
    // Mmm, Some("eggs").
}

fn some_breakfast() -> impl Iterator<Item = Option<&'static str>> {
    vec!["bacon", "sausages", "hash browns", "eggs"]
        .into_iter()
        .map(Some)
}

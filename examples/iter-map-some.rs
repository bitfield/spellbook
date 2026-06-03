fn main() {
    for food in ["bacon", "sausages", "hash browns", "eggs"]
        .iter()
        .map(Some)
    {
        println!("Mmm, {food:?}.");
    }
    // Mmm, Some("bacon").
    // Mmm, Some("sausages").
    // Mmm, Some("hash browns").
    // Mmm, Some("eggs").
}

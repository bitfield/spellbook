#[expect(clippy::min_ident_chars, reason = "reduce line width")]
fn main() {
    let breakfast = [
        "bacon and eggs",
        "hash browns",
        "sausages",
        "bacon sandwich",
    ];
    for dish in breakfast.iter().filter_map(|f| {
        f.starts_with("bacon")
            .then_some(format!("{f} with hot sauce"))
    }) {
        println!("{dish}");
    }
    // bacon and eggs with hot sauce
    // bacon sandwich with hot sauce
}

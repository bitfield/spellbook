fn main() {
    let breakfast = [
        "bacon and eggs",
        "hash browns",
        "sausages",
        "bacon sandwich",
    ];
    for dish in breakfast.iter().filter_map(|food| {
        food.starts_with("bacon")
            .then_some(format!("{food} with hot sauce"))
    }) {
        println!("{dish}");
    }
    // bacon and eggs with hot sauce
    // bacon sandwich with hot sauce
}

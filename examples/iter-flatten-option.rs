fn main() {
    for sweet in [
        Some("chocolate"),
        None,
        Some("candy"),
        Some("gummy"),
        None,
        None,
        Some("toffee"),
    ]
    .iter()
    .flatten()
    {
        println!("{sweet}");
    }
    // chocolate
    // candy
    // gummy
    // toffee
}

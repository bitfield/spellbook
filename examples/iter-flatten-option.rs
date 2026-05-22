fn main() {
    let sweets = [
        Some("chocolate"),
        None,
        Some("candy"),
        Some("gummy"),
        None,
        None,
        Some("toffee"),
    ];
    for sweet in sweets.iter().flatten() {
        println!("{sweet}");
    }
    // chocolate
    // candy
    // gummy
    // toffee
}

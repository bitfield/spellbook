#[expect(clippy::min_ident_chars, reason = "reduce line width")]
fn main() {
    let results = [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
        Err("out of stock"),
    ];
    for nosh in results.iter().filter_map(|r| r.ok()) {
        println!("{nosh:?}");
    }
    // "hot dog"
    // "burger"
    // "fries"

    for nosh in results.iter().flatten() {
        println!("{nosh:?}");
    }
    // "hot dog"
    // "burger"
    // "fries"
}

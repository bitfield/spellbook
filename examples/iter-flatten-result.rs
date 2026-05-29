fn main() {
    let results = [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
        Err("out of stock"),
    ];
    for nosh in results.iter().filter_map(|food| food.ok()) {
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

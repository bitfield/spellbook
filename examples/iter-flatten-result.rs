fn main() {
    for nosh in [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
        Err("out of stock"),
    ]
    .iter()
    .filter_map(|food| food.ok())
    {
        println!("{nosh:?}");
    }
    // "hot dog"
    // "burger"
    // "fries"

    for nosh in [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
        Err("out of stock"),
    ]
    .iter()
    .flatten()
    {
        println!("{nosh:?}");
    }
    // "hot dog"
    // "burger"
    // "fries"
}

fn main() {
    let lunch: Result<Vec<&str>, &str> = [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
    ]
    .into_iter()
    .collect();
    println!("{lunch:?}");
    // Err("out of stock")
}

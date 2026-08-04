fn main() {
    let mut maybe_lasagna = Some("Lasagna");
    let mut dinner = maybe_lasagna.unwrap_or("Hot Pockets");
    println!("{dinner}");
    // Lasagna
    maybe_lasagna = None;
    dinner = maybe_lasagna.unwrap_or("Hot Pockets");
    println!("{dinner}");
    // Hot Pockets
}

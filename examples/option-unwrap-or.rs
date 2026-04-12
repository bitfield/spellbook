#[expect(clippy::unnecessary_literal_unwrap, reason = "example")]
fn main() {
    let maybe_lasagna = Some("Lasagna");
    let dinner = maybe_lasagna.unwrap_or("Hot Pockets");
    println!("{dinner}");
    // Lasagna
}

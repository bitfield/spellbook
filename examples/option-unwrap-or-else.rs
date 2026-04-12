#[expect(clippy::unnecessary_literal_unwrap, reason = "example")]
fn main() {
    let emergency_dinner = "Order pizza";
    let maybe_lasagna = None;
    let dinner = maybe_lasagna.unwrap_or_else(|| {
        println!("Uh-oh, we're out of lasagna.");
        emergency_dinner
    });
    println!("{dinner}");
    // Uh-oh, we're out of lasagna.
    // Order pizza
}

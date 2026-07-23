#[expect(clippy::shadow_unrelated, reason = "different approaches")]
fn main() {
    let value: f64 = 9.251;
    println!("Please pay the bearer ${value:.2}");
    // Please pay the bearer $9.25

    let value: f64 = 4.995;
    println!("Please pay the bearer ${value:.2}");
    let value: f64 = -4.995;
    println!("Your balance is now ${value:.2}");
}

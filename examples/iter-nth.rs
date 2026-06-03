#[expect(clippy::iter_nth, reason = "demo")]
fn main() {
    println!("{:?}", ["popcorn", "crisps", "nuts"].iter().nth(2));
    // Some("nuts")
}

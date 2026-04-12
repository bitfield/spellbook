#[expect(clippy::unnecessary_literal_unwrap, reason = "example")]
fn main() {
    let maybe_guests = Some(3);
    let guests = maybe_guests.unwrap_or_default();
    if guests > 0 {
        println!("Hope there's enough lasagna for {guests} guests");
    }
}

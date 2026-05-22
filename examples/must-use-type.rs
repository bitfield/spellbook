#[expect(unused_must_use, reason = "example")]
#[expect(dead_code, reason = "example")]
#[expect(clippy::no_effect, reason = "example")]
fn main() {
    #[must_use]
    pub struct Payload(pub i32);

    Payload(42);
}

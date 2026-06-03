#[must_use]
pub fn compute() -> i32 {
    2 * 2
}

#[expect(clippy::let_underscore_must_use, reason = "match compiler suggestion")]
#[expect(clippy::let_underscore_untyped, reason = "match compiler suggestion")]
fn main() {
    let _ = compute();
}

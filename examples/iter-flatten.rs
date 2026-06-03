#[expect(clippy::map_flatten, reason = "example")]
fn main() {
    for ch in ["bacon", "eggs", "waffles"]
        .into_iter()
        .map(str::chars)
        .flatten()
    {
        print!("{ch} ");
    }
    // b a c o n e g g s w a f f l e s
}

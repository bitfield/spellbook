#[expect(clippy::map_flatten, reason = "example")]
fn main() {
    let breakfast = ["bacon", "eggs", "waffles"];
    for ch in breakfast.into_iter().map(str::chars).flatten() {
        print!("{ch} ");
    }
    // b a c o n e g g s w a f f l e s
}

fn main() {
    let breakfast = ["bacon", "eggs", "waffles"];
    for dish in breakfast.into_iter().map(str::chars) {
        for ch in dish {
            print!("{ch} ");
        }
    }
    // b a c o n e g g s w a f f l e s
}

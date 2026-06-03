fn main() {
    for ch in ["bacon", "eggs", "waffles"]
        .into_iter()
        .flat_map(str::chars)
    {
        print!("{ch} ");
    }
    // b a c o n e g g s w a f f l e s
}

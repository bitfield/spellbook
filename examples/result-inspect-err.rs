#[expect(clippy::print_stderr, reason = "simplicity")]
fn main() {
    let possible_cake = try_bake().inspect_err(|err| {
        eprintln!("It went... {err}.");
    });
    // It went... poorly.

    if let Ok(cake) = possible_cake {
        println!("One slice of {cake} or two?");
    }
}

fn try_bake() -> Result<&'static str, &'static str> {
    Err("poorly")
}

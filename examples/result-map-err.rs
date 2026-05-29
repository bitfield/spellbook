#[expect(clippy::print_stderr, reason = "example")]
fn main() {
    try_bake()
        .map_err(|err| format!("It went... {err}."))
        .unwrap_or_else(|err| eprintln!("{err}"));
}

#[expect(clippy::unnecessary_wraps, reason = "example")]
fn try_bake() -> Result<(), &'static str> {
    Ok(())
}

fn main() {
    try_bake()
        .map_err(|e| format!("It went... {e}."))
        .unwrap_or_else(|e| eprintln!("{e}"));
}

#[expect(clippy::unnecessary_wraps, reason = "example")]
fn try_bake() -> Result<(), &'static str> {
    Ok(())
}

use spellbook::Burger;

#[expect(clippy::let_underscore_untyped, reason = "example")]
fn main() {
    let _ = Burger; // mmm, burger
}

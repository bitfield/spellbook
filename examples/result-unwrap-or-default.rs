use std::fmt::Display;

struct Snack(&'static str);

impl Default for Snack {
    fn default() -> Self {
        Self("whatever's in the fridge")
    }
}

impl Display for Snack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[expect(clippy::unnecessary_literal_unwrap, reason = "example")]
fn main() {
    let possible_snack: Result<Snack, _> = Err("forgot to buy groceries");
    let snack = possible_snack.unwrap_or_default();
    println!("Hope you're in the mood for {snack}");
    // Hope you're in the mood for whatever's in the fridge
}

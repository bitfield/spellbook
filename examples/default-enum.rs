#[expect(clippy::exhaustive_enums, reason = "clarity")]
#[derive(Debug, Default)]
pub enum Cocktail {
    Margarita,
    #[default]
    Martini,
    Mojito,
}

fn main() {
    let drink = Cocktail::default();
    println!("Time for a {drink:?} before dinner?");
    // Time for a Martini before dinner?
}

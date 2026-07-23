#![expect(clippy::shadow_unrelated, reason = "example")]

#[derive(Debug)]
pub struct Sandwich {
    pub ingredients: Vec<String>,
}

impl From<Vec<String>> for Sandwich {
    fn from(ingredients: Vec<String>) -> Self {
        Self { ingredients }
    }
}

fn main() {
    let ingredients = vec!["chicken".to_owned(), "mayo".to_owned()];
    let sandwich = Sandwich::from(ingredients);
    println!("{sandwich:?}");
    // Sandwich { ingredients: ["chicken", "mayo"] }

    let ingredients = vec!["cheese".to_owned(), "pickle".to_owned()];
    let sandwich: Sandwich = ingredients.into();
    println!("{sandwich:?}");
    // Sandwich { ingredients: ["cheese", "pickle"] }
}

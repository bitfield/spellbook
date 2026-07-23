#[non_exhaustive]
pub struct Meal {
    pub name: String,
}

impl From<&str> for Meal {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

fn invite(meal: impl Into<Meal>) {
    println!("You're invited to a meal of {}", meal.into().name);
}

fn main() {
    invite("paella");
    // You're invited to a meal of paella
}

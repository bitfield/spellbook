#[non_exhaustive]
#[derive(Debug)]
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

fn main() {
    let meal = Meal::from("sausage & eggs");
    println!("{meal:?}");
    // Meal { name: "sausage & eggs" }
}

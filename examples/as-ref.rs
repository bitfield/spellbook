#[non_exhaustive]
#[derive(Debug)]
pub struct Meal {
    pub name: String,
}

impl AsRef<str> for Meal {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn special(fish: impl AsRef<str>) {
    let fish_str = fish.as_ref();
    println!("Today's fish is {fish_str}. Enjoy your meal.");
}

fn main() {
    let meal = Meal {
        name: "Trout à la Crème".to_owned(),
    };
    special(&meal);
    // Today's fish is Trout à la Crème. Enjoy your meal.
}

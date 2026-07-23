#[expect(clippy::arbitrary_source_item_ordering, reason = "logical order")]
#[non_exhaustive]
#[derive(Debug)]
pub struct Meal {
    pub starter: Option<&'static str>,
    pub main: &'static str,
    pub dessert: Option<&'static str>,
}

impl Meal {
    #[must_use]
    pub fn new(main: &'static str) -> Self {
        Self {
            starter: None,
            main,
            dessert: None,
        }
    }
}

fn main() {
    let mut meal = Meal::new("fish & chips");
    meal.dessert = Some("summer pudding");
    println!("{meal:#?}");
    // Meal {
    //     starter: None,
    //     main: "fish & chips",
    //     dessert: Some(
    //         "summer pudding",
    //     ),
    // }
}

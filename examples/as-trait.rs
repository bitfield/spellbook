pub trait Breakfast {
    fn eat(&self) {
        println!("Mmm, nourishing breakfast.");
    }
}

pub trait Lunch {
    fn eat(&self) {
        println!("Mmm, tasty lunch.");
    }
}

pub struct Sandwich;

impl Breakfast for Sandwich {}
impl Lunch for Sandwich {}

fn main() {
    let sandwich = Sandwich;
    Breakfast::eat(&sandwich);
    // Mmm, nourishing breakfast.
    <Sandwich as Lunch>::eat(&sandwich);
    // Mmm, tasty lunch.
}

#[expect(clippy::absolute_paths, reason = "explicitness")]
fn main() {
    let fillings = ["ham", "cheese", "chicken"];
    let toppings = ["mustard", "pickle", "salsa"];
    for (filling, topping) in std::iter::zip(fillings, toppings) {
        println!("{filling} and {topping} sandwich");
    }
    // ham and mustard sandwich
    // cheese and pickle sandwich
    // chicken and salsa sandwich
}

use std::iter;

fn main() {
    let fillings = ["ham", "cheese", "chicken"];
    let toppings = ["mustard", "pickle", "salsa"];
    for (filling, topping) in iter::zip(fillings, toppings) {
        println!("{filling} and {topping} sandwich");
    }
    // ham and mustard sandwich
    // cheese and pickle sandwich
    // chicken and salsa sandwich
}

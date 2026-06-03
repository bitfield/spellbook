use core::fmt::{Display, Formatter, Result};

pub struct Sandwich {
    pub condiment: &'static str,
    pub filling: &'static str,
    pub topping: &'static str,
}

impl Display for Sandwich {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} and {} sandwich with {}",
            self.filling, self.topping, self.condiment
        )
    }
}

fn main() {
    let sandwich = Sandwich {
        condiment: "mayonnaise",
        filling: "chicken",
        topping: "bacon",
    };
    println!("{sandwich}");
    // chicken and bacon sandwich with mayonnaise
}

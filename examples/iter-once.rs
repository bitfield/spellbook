use std::iter::{once, once_with};

use spellbook::Soup;

fn main() {
    let one_soup = once(Soup);
    println!("{:?}", one_soup.collect::<Vec<_>>());
    // [Soup]

    let one_cake = once_with(|| "Ca".to_string() + "ke");
    println!("{:?}", one_cake.collect::<Vec<_>>());
    // ["Cake"]
}

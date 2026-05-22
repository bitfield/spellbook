use std::iter::empty;

use spellbook::Soup;

fn main() {
    let mut no_soup = empty::<Soup>();
    println!("{:?}", no_soup.next());
    // None
}

use std::iter;

use spellbook::Soup;

#[expect(clippy::string_add, reason = "simplicity")]
fn main() {
    let soups: Vec<_> = iter::once(Soup).collect();
    println!("{soups:?}");
    // [Soup]

    let cakes: Vec<_> = iter::once_with(|| "Ca".to_owned() + "ke").collect();
    println!("{cakes:?}");
    // ["Cake"]
}

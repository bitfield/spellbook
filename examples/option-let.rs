use spellbook::Soup;

fn main() {
    let maybe_soup = Some(Soup);
    if let Some(actual_soup) = maybe_soup {
        println!("{actual_soup:?}");
    } else {
        println!("No soup for you");
    }
    // Soup
}

use spellbook::Soup;

fn main() {
    let maybe_soup = Some(Soup);
    if let Some(soup) = maybe_soup {
        println!("{soup}");
    } else {
        println!("No soup for you");
    }
    // Soup
}

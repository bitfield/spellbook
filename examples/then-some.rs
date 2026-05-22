use spellbook::Soup;

fn main() {
    let deserves_soup = true;
    let maybe_soup: Option<Soup> = deserves_soup.then_some(Soup);
    println!("{maybe_soup:?}");
    // Some(Soup)
}

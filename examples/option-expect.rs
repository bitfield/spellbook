use spellbook::Soup;

#[expect(clippy::unnecessary_literal_unwrap, reason = "example")]
fn main() {
    let maybe_soup: Option<Soup> = Some(Soup);
    println!("{:?}", maybe_soup.expect("No soup for you"));
    // Soup
}

use spellbook::Soup;

#[expect(clippy::absolute_paths, reason = "clarity")]
fn main() {
    let mut no_soup = std::iter::empty::<Soup>();
    println!("{:?}", no_soup.next());
    // None
}

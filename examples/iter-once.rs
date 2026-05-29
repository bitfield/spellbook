use spellbook::Soup;

#[expect(clippy::absolute_paths, reason = "clarity")]
#[expect(clippy::string_add, reason = "simplicity")]
fn main() {
    let one_soup = std::iter::once(Soup);
    println!("{:?}", one_soup.collect::<Vec<_>>());
    // [Soup]

    let one_cake = std::iter::once_with(|| "Ca".to_owned() + "ke");
    println!("{:?}", one_cake.collect::<Vec<_>>());
    // ["Cake"]
}

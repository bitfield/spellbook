use spellbook::Soup;

fn main() {
    if let Ok(soup) = try_making_soup() {
        println!("{soup}");
    } else {
        println!("No soup for you");
    }
}

#[expect(clippy::unnecessary_wraps, reason = "example")]
fn try_making_soup() -> Result<Soup, &'static str> {
    Ok(Soup)
}

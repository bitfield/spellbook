use spellbook::{Lunch, Soup};

fn main() {
    if let Some(lunch) = maybe_get_lunch() {
        println!("{lunch}");
    }
}

fn maybe_get_lunch() -> Option<Lunch> {
    let soup = maybe_make_soup()?;
    Some(Lunch(soup))
}

#[expect(clippy::unnecessary_wraps, reason = "example")]
fn maybe_make_soup() -> Option<Soup> {
    Some(Soup)
}

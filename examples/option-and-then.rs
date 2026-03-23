use spellbook::{Lunch, Soup};

fn main() {
    let maybe_soup = Some(Soup);
    let maybe_lunch = maybe_soup.and_then(maybe_make_lunch);
    println!("Today's menu: {maybe_lunch:?}");
    // Today's menu: Some(Lunch(Soup))
}

#[expect(clippy::unnecessary_wraps, reason = "example")]
fn maybe_make_lunch(soup: Soup) -> Option<Lunch> {
    Some(Lunch(soup))
}

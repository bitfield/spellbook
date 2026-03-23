use spellbook::{Lunch, Soup};

fn main() {
    let maybe_soup = Some(Soup);
    let maybe_lunch = maybe_soup.map(definitely_make_lunch);
    println!("Today's menu: {maybe_lunch:?}");
    // Today's menu: Some(Lunch(Soup))
}

fn definitely_make_lunch(soup: Soup) -> Lunch {
    Lunch(soup)
}

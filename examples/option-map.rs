use spellbook::{Lunch, Soup};

fn main() {
    let maybe_soup = Some(Soup);
    let maybe_lunch = maybe_soup.map(Lunch);
    println!("Today's menu: {maybe_lunch:?}");
    // Today's menu: Some(Lunch(Soup))

    let maybe_soup = Some(Soup);
    let maybe_lunch = maybe_soup.map(|soup| {
        println!("Looks like we have some {soup}");
        Lunch(soup)
    });
    println!("Today's menu: {maybe_lunch:?}");
    // Looks like we have some Soup
    // Today's menu: Some(Lunch(Soup))
}

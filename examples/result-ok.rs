use spellbook::{Lunch, Soup};

fn main() {
    if let Some(lunch) = maybe_get_lunch() {
        println!("{lunch}");
    }
}

fn maybe_get_lunch() -> Option<Lunch> {
    let soup = try_make_soup().ok()?;
    Some(Lunch(soup))
}

fn try_make_soup() -> Result<Soup, &'static str> {
    let ingredients = Some(Soup);
    if let Some(soup) = ingredients {
        Ok(soup)
    } else {
        Err("oh no")
    }
}

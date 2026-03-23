use spellbook::Soup;

fn main() {
    if let Err(problem) = lunch_if_possible() {
        eprintln!("{problem}");
    }
}

fn lunch_if_possible() -> Result<(), String> {
    let soup = try_making_soup()?;
    println!("Yummy {soup:?}!");
    Ok(())
}

fn try_making_soup() -> Result<Soup, String> {
    Err("Whoops, spilt the soup".into())
}

use spellbook::Soup;

fn main() {
    match try_making_soup() {
        Ok(lunch) => println!("Mmm. {lunch}."),
        Err(problem) => eprintln!("{problem}"),
    }
}

fn try_making_soup() -> Result<Soup, &'static str> {
    let maybe_soup = None;
    maybe_soup.ok_or_else(|| {
        println!("I'm afraid there's been a problem in the kitchen");
        "Whoops, spilt the soup"
    })
}

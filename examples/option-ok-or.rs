use spellbook::Soup;

fn main() {
    match try_making_soup() {
        Ok(lunch) => println!("Mmm. {lunch}."),
        Err(problem) => eprintln!("{problem}"),
    }
}

fn try_making_soup() -> Result<Soup, &'static str> {
    let maybe_soup = Some(Soup);
    maybe_soup.ok_or("Whoops, spilt the soup")
}

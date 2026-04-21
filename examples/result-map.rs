use spellbook::Soup;

#[derive(Debug)]
struct Supper(Soup);

fn main() {
    let possible_soup: Result<Soup, &'static str> = Ok(Soup);
    let possible_supper = possible_soup.map(|soup| {
        println!("Steaming bowl of {soup} on the way.");
        Supper(soup)
    });
    println!("{:?}", possible_supper.expect("where's my soup?"));
    // Steaming bowl of Soup on the way.
    // Supper(Soup)
}

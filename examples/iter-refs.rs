use spellbook::Soup;

fn main() {
    let two_soups = [Soup, Soup];
    let mut soup_iter = two_soups.iter(); // not `into_iter`
    println!("{:?}", soup_iter.next());
    // Some(Soup)
    println!("{:?}", soup_iter.next());
    // Some(Soup)
    println!("{:?}", soup_iter.next());
    // None
    println!("{two_soups:?}");
    // [Soup, Soup]

    for soup in &two_soups {
        println!("{soup}");
    }
    println!("{two_soups:?}");
    // [Soup, Soup]
}

use spellbook::Soup;

fn main() {
    let deserves_soup = true;
    let maybe_soup = deserves_soup.then(Soup::new);
    println!("{maybe_soup:?}");
    // Some(Soup)

    let ingredient = "tomato";
    let maybe_soup = deserves_soup.then(|| {
        println!("Making {ingredient} soup...");
        Soup::from(ingredient)
    });
    println!("{maybe_soup:?}");
    // Making tomato soup...
    // Some(Soup)
    let _ = ingredient;

    println!("{:?}", false.then(|| unreachable!()));
    // None

    let done = 12;
    let total = 20;
    let percent = if total != 0 { done * 100 / total } else { 0 };
    println!("{percent}% done");
    // 60% done
}

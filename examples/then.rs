use spellbook::Soup;

#[expect(clippy::shadow_unrelated, reason = "showing different approaches")]
#[expect(clippy::unreachable, reason = "example")]
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

    println!("{:?}", false.then(|| unreachable!()));
    // None

    let done = 12_i32;
    let total = 20_i32;
    let percent = if total != 0_i32 {
        done.strict_mul(100_i32).strict_div(total)
    } else {
        0_i32
    };
    println!("{percent}% done");
    // 60% done
}

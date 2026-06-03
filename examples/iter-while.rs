#[expect(clippy::while_let_on_iterator, reason = "example")]
fn main() {
    let mut snacks = ["popcorn", "crisps", "nuts"].into_iter();
    while let Some(snack) = snacks.next() {
        println!("{snack}");
    }
    // popcorn
    // crisps
    // nuts
}

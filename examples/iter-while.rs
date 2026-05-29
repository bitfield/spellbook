#[expect(clippy::while_let_on_iterator, reason = "example")]
fn main() {
    let snacks = vec!["popcorn", "crisps", "nuts"];
    let mut snack_iter = snacks.into_iter();
    while let Some(snack) = snack_iter.next() {
        println!("{snack}");
    }
    // popcorn
    // crisps
    // nuts
}

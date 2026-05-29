fn main() {
    let snacks = vec!["popcorn", "crisps", "nuts"];
    let snack_iter = snacks.into_iter();
    for snack in snack_iter.take(2) {
        println!("{snack}");
    }
    // popcorn
    // crisps
}

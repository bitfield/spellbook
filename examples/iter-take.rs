fn main() {
    let snacks = vec!["popcorn", "crisps", "nuts"].into_iter();
    for snack in snacks.take(2) {
        println!("{snack}");
    }
    // popcorn
    // crisps
}

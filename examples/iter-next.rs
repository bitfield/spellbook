fn main() {
    let snacks = vec!["popcorn", "crisps", "nuts"];
    let mut snack_iter = snacks.into_iter();
    println!("{:?}", snack_iter.next());
    // Some("popcorn")
    println!("{:?}", snack_iter.next());
    // Some("crisps")
    println!("{:?}", snack_iter.next());
    // Some("nuts")
    println!("{:?}", snack_iter.next());
    // None
}

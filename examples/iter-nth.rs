fn main() {
    let snacks = vec!["popcorn", "crisps", "nuts"];
    let mut snack_iter = snacks.into_iter();
    println!("{:?}", snack_iter.nth(2));
    // Some("nuts")
}

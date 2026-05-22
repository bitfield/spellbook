fn main() {
    let mut snacks = vec!["popcorn", "crisps", "nuts"].into_iter();
    println!("{:?}", snacks.nth(2));
    // Some("nuts")
}

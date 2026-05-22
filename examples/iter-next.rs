fn main() {
    let mut snacks = vec!["popcorn", "crisps", "nuts"].into_iter();
    println!("{:?}", snacks.next());
    // Some("popcorn")
    println!("{:?}", snacks.next());
    // Some("crisps")
    println!("{:?}", snacks.next());
    // Some("nuts")
    println!("{:?}", snacks.next());
    // None
}

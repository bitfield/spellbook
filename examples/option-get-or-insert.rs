fn main() {
    let mut chefs_table = None;
    chefs_table.get_or_insert("John");
    println!("{chefs_table:?}");
    // Some("John")
    chefs_table.get_or_insert("Interloper");
    println!("{chefs_table:?}");
    // Some("John")
}

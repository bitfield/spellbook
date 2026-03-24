fn main() {
    use std::collections::HashMap;

    let mut products = HashMap::new();
    products.entry("SKU-001").or_insert("No name");
    println!("{products:#?}");
}

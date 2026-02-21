use std::collections::HashMap;

fn main() {
    let mut products = HashMap::new();
    products.entry("SKU-001").or_insert("No name");
    println!("{products:#?}");
}

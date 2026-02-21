use std::collections::HashMap;

fn main() {
    let mut doubles = HashMap::new();
    doubles.entry(42).or_insert_with_key(|k| k * 2);
    println!("{doubles:#?}");
}

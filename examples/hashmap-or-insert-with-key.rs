fn main() {
    use std::collections::HashMap;

    let mut doubles = HashMap::new();
    doubles.entry(42).or_insert_with_key(|k| k * 2);
    println!("{doubles:#?}");
}

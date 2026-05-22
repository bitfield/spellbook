fn main() {
    use std::collections::HashMap;

    let mut doubles = HashMap::new();
    doubles
        .entry(42)
        .or_insert_with_key(|key: &usize| key.strict_mul(2));
    println!("{doubles:#?}");
}

fn main() {
    use std::collections::HashMap;

    let mut customers = HashMap::new();
    customers
        .entry("Joan Smith")
        .or_insert_with(uuid::Uuid::new_v4);
    println!("{customers:#?}");
}

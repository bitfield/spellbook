use uuid::Uuid;

use std::collections::HashMap;

fn main() {
    let mut customers = HashMap::new();
    customers.entry("Joan Smith").or_insert_with(Uuid::new_v4);
    println!("{customers:?}");
    // {"Joan Smith": 5878cb0c-35b8-4711-bc1f-6bfcf6e3a0f4}
}

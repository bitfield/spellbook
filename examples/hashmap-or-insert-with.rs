use uuid::Uuid;

use std::collections::HashMap;

fn main() {
    let mut customers = HashMap::new();
    customers.entry("Joan Smith").or_insert_with(Uuid::new_v4);
    println!("{customers:#?}");
}

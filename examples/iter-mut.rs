#[expect(
    clippy::shadow_unrelated,
    reason = "showing different approaches to same data"
)]
fn main() {
    let mut desserts = [
        "ice cream".to_string(),
        "cake".to_string(),
        "pudding".to_string(),
    ];
    let mut dessert_iter = desserts.iter_mut();
    if let Some(dish) = dessert_iter.next() {
        dish.push_str(" with chocolate sauce");
    }
    if let Some(dish) = dessert_iter.next() {
        dish.push_str(" with chocolate sauce");
    }
    if let Some(dish) = dessert_iter.next() {
        dish.push_str(" with chocolate sauce");
    }
    println!("{desserts:#?}");

    let mut desserts = [
        "ice cream".to_string(),
        "cake".to_string(),
        "pudding".to_string(),
    ];
    for dish in &mut desserts {
        dish.push_str(" with chocolate sauce");
    }
    println!("{desserts:#?}");
}

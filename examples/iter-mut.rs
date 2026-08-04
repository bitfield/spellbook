#[expect(
    clippy::shadow_unrelated,
    reason = "showing different approaches to same data"
)]
fn main() {
    let mut desserts = [
        "ice cream".to_owned(),
        "cake".to_owned(),
        "pudding".to_owned(),
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
    // [
    //     "ice cream with chocolate sauce",
    //     "cake with chocolate sauce",
    //     "pudding with chocolate sauce",
    // ]
    let mut desserts = [
        "ice cream".to_owned(),
        "cake".to_owned(),
        "pudding".to_owned(),
    ];
    for dish in &mut desserts {
        dish.push_str(" with chocolate sauce");
    }
    println!("{desserts:#?}");
    // [
    //     "ice cream with chocolate sauce",
    //     "cake with chocolate sauce",
    //     "pudding with chocolate sauce",
    // ]
}

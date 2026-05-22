fn main() {
    let maybe_bread = Some("bread".to_string());
    let maybe_cheese = Some("cheese".to_string());
    let snack = maybe_bread
        .zip(maybe_cheese)
        .map(|(a, b)| {
            println!("Making a snack from some {a} and {b}...");
            format!("Mmm, {a} and {b}")
        })
        .expect("Wizard needs food badly");
    println!("{snack}");
    // Making a snack from some bread and cheese...
    // Mmm, bread and cheese
}

fn main() {
    let values: [f64; _] = [4.5, -4.5, 5.5, -5.5];
    for val in values {
        println!("round() of {val} is {:.1}", val.round());
    }
    for val in values {
        println!(
            "round_ties_even() of {val} is {:.1}",
            val.round_ties_even()
        );
    }
}

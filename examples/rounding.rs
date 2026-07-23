fn main() {
    let values: [f64; _] = [4.5, -4.5, 5.5, -5.5];
    for val in values {
        println!("round() of {val} is {:.1}", val.round());
    }
    // round() of 4.5 is 5.0
    // round() of -4.5 is -5.0
    // round() of 5.5 is 6.0
    // round() of -5.5 is -6.0
    for val in values {
        println!("round_ties_even() of {val} is {:.1}", val.round_ties_even());
    }
    // round_ties_even() of 4.5 is 4.0
    // round_ties_even() of -4.5 is -4.0
    // round_ties_even() of 5.5 is 6.0
    // round_ties_even() of -5.5 is -6.0
}

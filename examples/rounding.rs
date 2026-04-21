fn main() {
    let values: [f64; _] = [4.5, -4.5, 5.5, -5.5];
    for v in values {
        println!("The round() of {v} is {:.1}", v.round());
    }
    for v in values {
        println!("The round_ties_even() of {v} is {:.1}", v.round_ties_even());
    }
}

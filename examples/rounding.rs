fn main() {
    let value: f64 = 4.9;
    println!("The floor of {value} is {:.1}", value.floor());
    let value: f64 = 4.2;
    println!("The ceiling of {value} is {:.1}", value.ceil());
    let value: f64 = 4.5;
    println!("The round() of {value} is {:.1}", value.round());
    let value: f64 = 4.5;
    println!(
        "The round_ties_even() of {value} is {:.1}",
        value.round_ties_even()
    );
    let value: f64 = 5.5;
    println!(
        "whereas the round_ties_even() of {value} is {:.1}",
        value.round_ties_even()
    );
}

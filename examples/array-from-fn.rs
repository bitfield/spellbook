use std::array;

fn main() {
    let evens: [usize; 4] = array::from_fn(|num| num.strict_mul(2));
    println!("{evens:?}");
    // [0, 2, 4, 6]

    let cake = String::from("cake");
    let feast: [String; 100] = array::from_fn(|_| cake.clone());
    println!("{feast:?}");
    // ["cake", "cake", "cake", "cake", ... ]
}

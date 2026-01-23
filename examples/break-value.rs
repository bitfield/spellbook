use rand::prelude::*;

fn main() {
    println!("Looking for a value higher than 90...");
    let mut rng = rand::rng();
    let outlier = loop {
        let x = rng.random_range(0..100);
        if x > 90 {
            break x;
        }
    };
    println!("{outlier}");
}

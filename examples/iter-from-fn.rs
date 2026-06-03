use std::iter;

fn main() {
    let mut drinks = 0_usize;
    let mut barkeep = iter::from_fn(|| {
        if drinks < 4 {
            println!(">>> Coming right up");
            drinks = drinks.saturating_add(1);
            Some("booze")
        } else {
            println!(">>> Don't you think you've had enough?");
            None
        }
    });
    loop {
        println!("I demand to have some booze");
        if barkeep.next().is_none() {
            break;
        }
    }
}

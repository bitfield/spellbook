use std::iter::repeat_n;

fn main() {
    let eight_buns = repeat_n("bun", 8);
    for bun in eight_buns {
        print!("{bun} ");
    }
    println!("— Full up!");
    // bun bun bun bun bun bun bun bun — Full up!
}

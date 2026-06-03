use std::iter;

fn main() {
    for bun in iter::repeat_n("bun", 8) {
        print!("{bun} ");
    }
    println!("- Full up!");
    // bun bun bun bun bun bun bun bun - Full up!
}

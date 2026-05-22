use std::iter::repeat;

fn main() {
    let infinite_buns = repeat("bun");
    for bun in infinite_buns {
        print!("{bun} ");
    }
    // bun bun bun bun bun bun...
}

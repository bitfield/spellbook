use std::iter;

fn main() {
    for bun in iter::repeat("bun") {
        print!("{bun} ");
    }
    // bun bun bun bun bun bun...
}

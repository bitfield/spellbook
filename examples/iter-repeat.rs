#[expect(clippy::absolute_paths, reason = "clarity")]
fn main() {
    let infinite_buns = std::iter::repeat("bun");
    for bun in infinite_buns {
        print!("{bun} ");
    }
    // bun bun bun bun bun bun...
}

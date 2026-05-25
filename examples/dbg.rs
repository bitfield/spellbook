#[expect(clippy::dbg_macro, reason = "demonstrating dbg!")]
#[expect(clippy::shadow_unrelated, reason = "alternate approach")]
#[expect(clippy::integer_division, reason = "for simplicity")]
#[expect(clippy::integer_division_remainder_used, reason = "ditto")]
fn main() {
    let value = 2 * 2;
    dbg!(value);

    let value = 4;
    dbg!(value * 3 / 4);

    let square = dbg!(value * value);
    println!("Ta-da! Today's magic number is {square}");
}

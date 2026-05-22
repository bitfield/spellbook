#[expect(clippy::arithmetic_side_effects, reason = "we'll get there")]
fn main() {
    let val: u8 = 250;
    for i in 1..7 {
        let answer = val + i;
        println!("{val} + {i} = {answer}");
    }
}

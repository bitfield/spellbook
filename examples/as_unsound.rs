#[expect(clippy::cast_possible_truncation, reason = "showing what not to do")]
fn main() {
    let input: u16 = 300;
    // let output = input as u8;
    let output = input as u8;
    // println!("Output: {output}");
    println!("Output: {output}");
}

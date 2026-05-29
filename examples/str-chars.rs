#[expect(
    clippy::non_ascii_literal,
    reason = "demonstrating bytes vs chars"
)]
fn main() {
    let input = "Why nøt trei oür møøse burger?";
    for ch in input.chars() {
        print!("{ch} ");
    }
    // W h y   n ø t   t r e i   o ü r   m ø ø s e   b u r g e r ?
    println!();

    if let Some(ch) = input.chars().nth(14) {
        println!("{ch}");
    }
    // ü
}

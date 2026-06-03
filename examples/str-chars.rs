#[expect(clippy::non_ascii_literal, reason = "demonstrating bytes vs chars")]
fn main() {
    for ch in "Why nøt trei oür møøse burger?".chars() {
        print!("{ch} ");
    }
    // W h y   n ø t   t r e i   o ü r   m ø ø s e   b u r g e r ?
    println!();

    if let Some(ch) = "Why nøt trei oür møøse burger?".chars().nth(14) {
        println!("{ch}");
    }
    // ü
}

#[expect(
    clippy::non_ascii_literal,
    reason = "demonstrating bytes vs chars"
)]
#[expect(clippy::min_ident_chars, reason = "line width")]
fn main() {
    let input = "Why nøt trei oür møøse burger?";
    for ch in input.chars() {
        if ch == 'ü' {
            println!("Found a ü character"); // but where?
        }
    }
    // Found a ü character

    for (pos, ch) in input.char_indices() {
        if ch == 'ü' {
            println!("Found a ü at byte position {pos}");
        }
    }
    // Found a ü at byte position 15

    if let Some(ch) = input.get(15..).and_then(|s| s.chars().next()) {
        println!("{ch}");
    }
    // ü
}

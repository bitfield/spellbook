fn main() {
    println!("{}", first(&[1, 2, 3]));
}

/// first returns the first number in the input slice.
fn first(input: &[i32]) -> i32 {
    debug_assert!(!input.is_empty(), "input must not be empty");
    input[0]
}

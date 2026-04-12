fn main() {
    let maybe_name = Some("Rustine".into());
    print_len(&maybe_name);
    drop(maybe_name);
}

#[expect(clippy::ref_option, reason = "example")]
fn print_len(maybe_name: &Option<String>) {
    println!(
        "{}",
        maybe_name.as_ref().map(String::len).unwrap_or_default()
    );
}

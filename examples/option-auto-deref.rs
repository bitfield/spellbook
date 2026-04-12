fn main() {
    let maybe_name = Some("Rustine".into());
    print_len(&maybe_name);
    drop(maybe_name);
}

#[expect(clippy::ref_option, reason = "example")]
fn print_len(maybe_name: &Option<String>) {
    if let Some(name) = maybe_name {
        println!("{}", name.len());
    }
}

fn main() {}

#[test]
fn included_str_contains_expected_data() {
    const TEXT: &str = include_str!("static-data.txt");
    assert_eq!(TEXT, "Hello, world!");
}

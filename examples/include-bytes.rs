fn main() {}

#[test]
fn included_bytes_contain_expected_data() {
    let bytes = include_bytes!("static-data.txt");
    assert_eq!(
        bytes,
        &[
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21
        ]
    );
}

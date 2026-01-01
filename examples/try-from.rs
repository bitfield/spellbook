fn main() {
    let input: u16 = 300;
    if let Ok(output) = u8::try_from(input) {
        println!("Output: {output}");
    } else {
        println!("oh no");
    }

    let input: u16 = 200;
    let _res = u8::try_from(input);
    if let Ok(output) = u8::try_from(input) {
        println!("Output: {output}");
    } else {
        println!("oh no");
    }
}

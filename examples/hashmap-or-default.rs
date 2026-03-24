fn main() {
    use std::collections::HashMap;

    let mut registers = HashMap::from([("X", 0_u8)]);
    let value = registers.entry("Y").or_default();
    *value = value.wrapping_add(1);
    println!("{registers:#?}");
}

use std::collections::HashMap;

fn main() {
    let mut registers = HashMap::from([("X", 0_u8)]);
    let value = registers.entry("Y").or_default();
    *value = value.wrapping_add(1);
    println!("{registers:#?}");
}

use std::fmt::UpperHex;

fn print_as_hex<T: UpperHex>(val: T) {
    println!("{val:#04X}");
}

pub fn main() {
    print_as_hex(1_u8);
    // 0x01
    print_as_hex(1_i32);
    // 0x01
    print_as_hex(1_u64);
    // 0x01
}

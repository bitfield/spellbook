use anyhow::Result;

use std::io::Read as _;

fn main() -> Result<()> {
    let buffer = &[0_u8; 8];
    for byte in buffer.bytes() {
        print!("{:#04X} ", byte?);
    }
    // 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00
    Ok(())
}

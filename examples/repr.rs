pub struct Rust {
    pub little_bear: u8,
    pub middle_bear: u16,
    pub big_bear: u32,
}

#[repr(C)]
pub struct C {
    pub little_bear: u8,
    pub middle_bear: u16,
    pub big_bear: u32,
}

fn main() {}

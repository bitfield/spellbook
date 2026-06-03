#[expect(clippy::arbitrary_source_item_ordering, reason = "demonstrating repr")]
pub struct Goldilocks {
    pub little_bear: u8,
    pub middle_bear: u16,
    pub big_bear: u32,
}

fn main() {}

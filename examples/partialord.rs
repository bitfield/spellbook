#![expect(
    clippy::arbitrary_source_item_ordering,
    reason = "shows derived ordering"
)]
#![expect(clippy::missing_assert_message, reason = "clarity")]

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Cake {
    pub kind: &'static str,
    pub frosting: &'static str,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Muffin {
    ChocolateChip,
    Banana,
    Blueberry,
}

#[non_exhaustive]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Pastry {
    Éclair = 3,
    Croissant = 1,
    CreamHorn = 2,
}

fn main() {
    assert!(
        Cake {
            kind: "chocolate",
            frosting: "orange",
        } < Cake {
            kind: "fruit",
            frosting: "lemon",
        },
        "chocolate cakes should always come first"
    );

    assert!(
        Muffin::ChocolateChip < Muffin::Banana,
        "primacy of chocolate has been violated"
    );

    assert!(Pastry::Croissant < Pastry::CreamHorn);
    assert!(Pastry::CreamHorn < Pastry::Éclair);
}

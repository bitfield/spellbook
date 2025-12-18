mod snacks {
    pub struct Popcorn;
    pub(crate) struct Chocolate;
}

#[expect(unused_variables, reason = "example")]
fn main() {
    let yum = snacks::Popcorn;
    let nosh = snacks::Chocolate;
}

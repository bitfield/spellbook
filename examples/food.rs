pub struct RoastBeef;

mod snacks {
    pub struct Popcorn;
    #[expect(dead_code, reason = "example")]
    pub(crate) struct Chocolate;
}

mod carbs {
    pub mod pasta {
        pub mod filled {
            pub struct Tortelloni;
        }
    }
}

pub mod prelude {
    pub use crate::RoastBeef;
    pub use crate::carbs::pasta::filled::Tortelloni;
    pub use crate::snacks::Popcorn;
}

fn main() {}

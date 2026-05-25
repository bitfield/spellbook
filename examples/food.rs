mod carbs {
    pub mod pasta {
        pub mod filled {
            pub struct Tortelloni;
        }
    }
}

#[expect(clippy::pub_use, reason = "demonstrating `pub use`")]
pub mod prelude {
    pub use crate::RoastBeef;
    pub use crate::carbs::pasta::filled::Tortelloni;
    pub use crate::snacks::Popcorn;
}

#[expect(clippy::pub_with_shorthand, reason = "simplicity")]
mod snacks {
    pub struct Popcorn;
    #[expect(dead_code, reason = "example")]
    pub(crate) struct Chocolate;
}

pub struct RoastBeef;

fn main() {}

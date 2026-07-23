use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct Fishfinger;

impl Display for Fishfinger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

fn main() {
    println!("{Fishfinger}");
    // Fishfinger
}

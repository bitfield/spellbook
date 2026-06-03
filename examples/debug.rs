use core::fmt::{Debug, Formatter, Result};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "You're at {}th Street and {}th Avenue.", self.x, self.y)
        } else {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

fn main() {
    let point = Point { x: 30, y: 25 };
    println!("{point:#?}");
}

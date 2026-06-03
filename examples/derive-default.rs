#[derive(Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let point = Point::default();
    println!("({}, {})", point.x, point.y);
}

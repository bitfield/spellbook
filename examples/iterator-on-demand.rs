struct Chicken;

impl Iterator for Chicken {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        Some("Egg")
    }
}

fn main() {
    let eggs: Vec<_> = Chicken.take(3).collect();
    println!("{eggs:?}");
    // ["Egg", "Egg", "Egg"]
}

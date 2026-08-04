pub struct Fridge(Vec<&'static str>);

impl Iterator for Fridge {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {
    let fridge = Fridge(vec!["butter", "milk", "cheese"]);
    for food in fridge {
        println!("We should probably eat up this {food}");
    }
    // We should probably eat up this cheese
    // We should probably eat up this milk
    // We should probably eat up this butter
}

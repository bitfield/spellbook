fn main() {
    for food in breakfast() {
        println!("Mmm, {food}.");
    }
}

fn breakfast() -> impl Iterator<Item = &'static str> {
    vec!["bacon", "sausages", "hash browns", "eggs"].into_iter()
}

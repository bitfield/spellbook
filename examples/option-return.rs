fn main() {
    if let Some(val) = first(&[1, 2, 3]) {
        println!("{val}");
    } else {
        println!("Looks like that list is empty!");
    }
}

fn first<T>(list: &[T]) -> Option<&T> {
    if let [first, ..] = list {
        Some(first)
    } else {
        None
    }
}

use std::collections::HashMap;

fn main() {
    let crew = HashMap::from([
        (1, "Jack Aubrey"),
        (2, "Stephen Maturin"),
        (3, "Tom Pullings"),
    ]);
    let Some(_stephen) = crew.get(&2) else {
        panic!("Stephen is not aboard");
    };
}

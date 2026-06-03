use spellbook::Soup;

fn main() {
    use std::iter;

    println!("{:?}", iter::empty::<Soup>().next());
    // None
}

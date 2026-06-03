#[expect(clippy::arbitrary_source_item_ordering, reason = "more logical")]
fn main() {
    enum Option<T> {
        Yep(T),
        Nope,
    }

    if let Option::Yep(this_works) = Option::Yep("this is fine") {
        println!("Apparently {this_works}!");
    }
    let _: Option<i32> = Option::Nope;
}

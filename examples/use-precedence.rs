fn main() {
    enum Option<T> {
        Yep(T),
        Nope,
    }

    if let Option::Yep(s) = Option::Yep("this is fine") {
        println!("Apparently {s}!");
    }
    let _: Option<i32> = Option::Nope;
}

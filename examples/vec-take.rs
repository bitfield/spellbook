fn main() {
    use std::mem;
    let mut buf = Vec::new();
    buf.push("golden idol");
    let idol = mem::take(&mut buf);
    buf.push("bag of sand");
    println!("{idol:?}, {buf:?}");
}

fn main() {
    let mut buf = Vec::new();
    buf.push("golden idol");
    let idol = std::mem::take(&mut buf);
    buf.push("bag of sand");
    println!("{idol:?}, {buf:?}");
}

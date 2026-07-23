#[expect(clippy::absolute_paths, reason = "explicitness")]
fn main() {
    let mut buf = Vec::new();
    buf.push("golden idol");
    let idol = std::mem::take(&mut buf);
    buf.push("bag of sand");
    println!("{idol:?}, {buf:?}");
    // ["golden idol"], ["bag of sand"]
}

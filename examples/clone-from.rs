#[derive(Debug)]
pub struct Buffer(Vec<u8>);

impl Clone for Buffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

#[expect(unused, reason = "example")]
#[expect(clippy::assigning_clones, reason = "showing what not to do")]
fn main() {
    let mut buf1 = Buffer(vec![0; 8]);
    let buf2 = Buffer(vec![1; 8]);
    buf1 = buf2.clone();
    println!("cloned buf1: {buf1:?}");
    buf1.clone_from(&buf2);
    println!("cloned buf1: {buf1:?}");
}

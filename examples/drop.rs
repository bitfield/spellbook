pub struct Pizza;

impl Drop for Pizza {
    fn drop(&mut self) {
        println!("Oh no. The cheese went everywhere.");
    }
}

fn main() {
    let _pie = Pizza;
}

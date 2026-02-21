fn main() {
    let eggs = {
        let mut eggs: i32 = 58;
        if let Some(increase) = price_rise() {
            eggs = eggs.strict_add(increase);
        }
        eggs
    };
    println!("{eggs}");
}

#[expect(clippy::unnecessary_wraps, reason = "example")]
fn price_rise() -> Option<i32> {
    Some(3)
}

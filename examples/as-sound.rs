use std::{thread::sleep, time::Duration};

fn main() {
    wait_millis(500);
}

#[expect(clippy::cast_lossless, reason = "example")]
#[expect(clippy::as_conversions, reason = "example")]
#[expect(clippy::shadow_reuse, reason = "typecasting")]
fn wait_millis(delay: u32) {
    let delay = delay as u64;
    sleep(Duration::from_millis(delay));
}

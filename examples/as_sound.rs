// #![expect(clippy::cast_lossless, reason = "example")]
use std::{thread::sleep, time::Duration};

fn main() {
    wait_millis(500);
}

fn wait_millis(delay: u32) {
    let delay = delay as u64;
    sleep(Duration::from_millis(delay));
}

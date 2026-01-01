use std::{thread::sleep, time::Duration};

fn main() {
    wait_millis(500);
}

fn wait_millis(delay: u32) {
    let delay = u64::from(delay);
    sleep(Duration::from_millis(delay));
}

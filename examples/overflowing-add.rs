fn main() {
    let val: u8 = 250;
    for i in 1..7 {
        let (answer, overflow) = val.overflowing_add(i);
        println!(
            "{val} + {i} = {answer} {}",
            if overflow {
                "(overflowed and wrapped)"
            } else {
                "(no overflow)"
            }
        );
    }
}

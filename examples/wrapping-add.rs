fn main() {
    let val: u8 = 250;
    for i in 1..7 {
        let answer = val.wrapping_add(i);
        println!("{val} + {i} = {answer}");
    }
}

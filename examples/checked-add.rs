fn main() {
    let val: u8 = 250;
    for i in 1..7 {
        print!("{val} + {i} = ");
        if let Some(answer) = val.checked_add(i) {
            println!("{answer}");
        } else {
            println!("oh no, overflow!");
        }
    }
}

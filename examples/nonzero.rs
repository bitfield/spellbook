use std::num::NonZero;

fn main() {
    let input = 5;
    let val = NonZero::<u8>::new(input).expect("must be non-zero");
    blow_up_on_zero(val);
}

fn blow_up_on_zero(val: NonZero<u8>) {
    if val.get() == 0 {
        unreachable!("Zero received, blowing up world");
    } else {
        println!("World escaped destruction... this time");
    }
}

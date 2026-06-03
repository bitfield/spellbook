fn main() {
    let mut val = 99_i32;
    reset(&mut val);
    println!("{val}");
}

fn reset<T: Default>(val: &mut T) {
    *val = T::default();
}

fn main() {
    use std::array;
    let a: [usize; 4] = array::from_fn(|i| i * 2);
    println!("{a:?}");
    // [0, 2, 4, 6]
    
    let mut nums = ['a', 'b', 'c', 'd'].into_iter();
    let a: [char; 4] = array::from_fn(|_| nums.next().unwrap());
    println!("{a:?}");
    // ['a', 'b', 'c', 'd']
}
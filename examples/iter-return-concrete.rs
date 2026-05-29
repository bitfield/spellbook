use std::{iter::Map, ops::RangeFrom};

fn main() {
    for even in evens() {
        print!("{even}, ");
    }
    // 0, 2, 4, 6, 8, ...
}

fn double(num: usize) -> usize {
    num.strict_mul(2)
}

fn evens() -> Map<RangeFrom<usize>, fn(usize) -> usize> {
    (0..).map(double)
}

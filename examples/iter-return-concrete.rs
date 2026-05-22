use std::{iter::Map, ops::RangeFrom};

fn main() {
    for n in evens() {
        print!("{n}, ");
    }
    // 0, 2, 4, 6, 8, ...
}

fn double(n: usize) -> usize {
    n.strict_mul(2)
}

fn evens() -> Map<RangeFrom<usize>, fn(usize) -> usize> {
    (0..).map(double)
}

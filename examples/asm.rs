use std::arch::asm;

fn main() {
    println!("{}", multiply(2, 2));
}

fn multiply(x: i32, y: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!("smull {result:x}, {x:w}, {y:w}",
            result = out(reg) result,
            x = in(reg) x,
            y = in(reg) y,
        );
    }
    result
}

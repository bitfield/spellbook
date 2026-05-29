fn main() {
    let snacks = vec!["popcorn", "crisps", "nuts"];
    let snack_iter = snacks.into_iter();
    for snack in snack_iter.skip(1) {
        println!("{snack}");
    }
    // crisps
    // nuts

    let nums = 0..10;
    for num in nums.skip(5).take(3) {
        print!("{num} ");
    }
    // 5 6 7
}

fn main() {
    for snack in ["popcorn", "crisps", "nuts"].iter().skip(1) {
        println!("{snack}");
    }
    // crisps
    // nuts

    for num in (0..10).skip(5).take(3) {
        print!("{num} ");
    }
    // 5 6 7
}

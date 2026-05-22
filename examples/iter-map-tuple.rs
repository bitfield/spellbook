fn main() {
    #[expect(dead_code, reason = "example")]
    #[derive(Debug)]
    struct UserId(u64);
    for id in (0..5).map(UserId) {
        print!("{id:?} ");
    }
    // [UserId(0) UserId(1) UserId(2) UserId(3) UserId(4)]
}

fn main() {
    #[expect(dead_code, reason = "example")]
    #[derive(Debug)]
    struct Vehicle {
        make: &'static str,
        model: &'static str,
        year: u16,
    }
    let car = Vehicle {
        make: "Land Rover",
        model: "Defender",
        year: 1991,
    };
    println!("{car:#?}");
    let slice = &[1, 2, 3];
    println!("{slice:#?}");
}

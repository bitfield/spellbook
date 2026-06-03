fn main() {
    for dish in [
        "egg and chips",
        "sandwich",
        "sausage and chips",
        "ice cream",
        "fish and chips",
        "lasagna",
    ]
    .iter()
    .filter(|food| food.contains("chips"))
    {
        println!("{dish}");
    }
    // egg and chips
    // sausage and chips
    // fish and chips

    for pos in [8_i32, -4, -7, 2, 0, 5, -14]
        .iter()
        .filter(|&&num| num >= 0)
    {
        print!("{pos} ");
    }
    // 8 2 0 5
}

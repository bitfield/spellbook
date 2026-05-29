fn main() {
    let foods = [
        "egg and chips",
        "sandwich",
        "sausage and chips",
        "ice cream",
        "fish and chips",
        "lasagna",
    ];
    for dish in foods.iter().filter(|food| food.contains("chips")) {
        println!("{dish}");
    }
    // egg and chips
    // sausage and chips
    // fish and chips

    let data = [8_i32, -4, -7, 2, 0, 5];
    for pos in data.iter().filter(|&&num| num >= 0) {
        print!("{pos} ");
    }
    // 8 2 0 5
}

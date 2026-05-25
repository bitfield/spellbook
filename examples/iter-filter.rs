#[expect(clippy::min_ident_chars, reason = "reduce line width")]
fn main() {
    let foods = [
        "egg and chips",
        "sandwich",
        "sausage and chips",
        "ice cream",
        "fish and chips",
        "lasagna",
    ];
    for dish in foods.iter().filter(|f| f.contains("chips")) {
        println!("{dish}");
    }
    // egg and chips
    // sausage and chips
    // fish and chips

    let data = [8_i32, -4, -7, 2, 0, 5];
    for pos in data.iter().filter(|&&n| n >= 0) {
        print!("{pos} ");
    }
    // 8 2 0 5
}

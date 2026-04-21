fn main() {
    let lunch = try_making_soup().unwrap_or_else(|err| {
        eprintln!("Bad news: {err}");
        "crackers"
    });
    println!("I guess we're having {lunch}");
    // Bad news: your housemate ate all the soup last night
    // I guess we're having crackers
}

fn try_making_soup() -> Result<&'static str, &'static str> {
    Err("your housemate ate all the soup last night")
}

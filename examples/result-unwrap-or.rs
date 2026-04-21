fn main() {
    let lunch = try_making_soup().unwrap_or("sandwiches");
    println!("I guess we're having {lunch}");
}

fn try_making_soup() -> Result<&'static str, &'static str> {
    Err("your housemate ate all the soup last night")
}

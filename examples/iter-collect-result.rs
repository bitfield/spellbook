#[expect(clippy::shadow_unrelated, reason = "different approaches")]
fn main() {
    let lunch: Result<Vec<&str>, &str> = [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
    ]
    .into_iter()
    .collect();
    println!("{lunch:?}");
    // Err("out of stock")

    let lunch: Result<Vec<&str>, &str> =
        [Ok("wings"), Ok("nuggets"), Ok("onion rings")]
            .into_iter()
            .collect();
    println!("{lunch:?}");
    // Ok(["wings", "nuggets", "onion rings"])
}

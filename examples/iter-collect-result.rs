#[expect(clippy::shadow_unrelated, reason = "different approaches")]
fn main() {
    let results = [
        Ok("hot dog"),
        Err("out of stock"),
        Ok("burger"),
        Ok("fries"),
    ];
    let lunch: Result<Vec<&str>, &str> = results.into_iter().collect();
    println!("{lunch:?}");
    // Err("out of stock")

    let results = [Ok("wings"), Ok("nuggets"), Ok("onion rings")];
    let lunch: Result<Vec<&str>, &str> = results.into_iter().collect();
    println!("{lunch:?}");
    // Ok(["wings", "nuggets", "onion rings"])
}

#![expect(clippy::iter_cloned_collect, reason = "example of `cloned`")]
fn main() {
    let chocs = [
        "caramel softy".to_owned(),
        "fudge duet".to_owned(),
        "white truffle".to_owned(),
    ];
    let cloned_chocs: Vec<String> = chocs.iter().cloned().collect();
    println!("Cloned chocs: {cloned_chocs:?}");
    // Cloned chocs: ["caramel softy", "fudge duet", "white truffle"]
    println!("Originals: {chocs:?}");
    // Originals: ["caramel softy", "fudge duet", "white truffle"]
}

fn main() {
    let chocs = [
        "caramel softy".to_owned(),
        "hazelnut swirl".to_owned(),
        "white truffle".to_owned(),
    ];
    let snack: Vec<String> = chocs.iter().take(2).cloned().collect();
    println!("{snack:?}");
    println!("{chocs:?}");
}

use std::env;

fn main() {
    let home = env::var("HOME").unwrap_or("No HOME set".into());
    println!("{home}");
}

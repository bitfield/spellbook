fn main() {
    let user = env!("USER");
    println!("This binary was built by {user}");
    let maybe_set = option_env!("MAYBE_SET");
    if let Some(value) = maybe_set {
        println!("MAYBE_SET was set to {value}");
    } else {
        println!("MAYBE_SET wasn't set, actually");
    }
}

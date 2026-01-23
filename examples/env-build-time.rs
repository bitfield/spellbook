fn main() {
    let user = env!("USER");
    println!("This binary was built by {user}");
    let maybe_email = option_env!("USER_EMAIL");
    if let Some(email) = maybe_email {
        println!("USER_EMAIL was set to {email}");
    } else {
        println!("USER_EMAIL wasn't set, but we'll get by");
    }
}

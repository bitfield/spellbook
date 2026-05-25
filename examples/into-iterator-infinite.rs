use std::iter::{Repeat, repeat};

struct InfiniteCookieJar;

impl IntoIterator for InfiniteCookieJar {
    type IntoIter = Repeat<Self::Item>;
    type Item = &'static str;

    fn into_iter(self) -> Self::IntoIter {
        repeat("Cookie")
    }
}

fn main() {
    for cookie in InfiniteCookieJar {
        println!("*crunches {cookie}*");
    }
}

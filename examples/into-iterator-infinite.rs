use std::iter::{self, Repeat};

struct InfiniteCookieJar;

impl IntoIterator for InfiniteCookieJar {
    type IntoIter = Repeat<Self::Item>;
    type Item = &'static str;

    fn into_iter(self) -> Self::IntoIter {
        iter::repeat("Cookie")
    }
}

fn main() {
    for cookie in InfiniteCookieJar {
        println!("*crunches {cookie}*");
    }
}

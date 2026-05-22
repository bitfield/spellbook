use std::array::IntoIter;

struct CookieJar;

impl IntoIterator for CookieJar {
    type Item = &'static str;

    type IntoIter = IntoIter<Self::Item, 10>;

    fn into_iter(self) -> IntoIter<&'static str, 10> {
        ["cookie"; 10].into_iter()
    }
}

use std::iter::{Repeat, repeat};

struct InfiniteCookieJar;

impl IntoIterator for InfiniteCookieJar {
    type Item = &'static str;

    type IntoIter = Repeat<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        repeat("Cookie")
    }
}

fn main() {
    for cookie in CookieJar {
        println!("*munches {cookie}*");
    }

    for cookie in InfiniteCookieJar {
        println!("*crunches {cookie}*");
    }
}

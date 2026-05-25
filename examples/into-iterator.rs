use std::array::IntoIter;

struct CookieJar;

impl IntoIterator for CookieJar {
    type IntoIter = IntoIter<Self::Item, 10>;
    type Item = &'static str;

    fn into_iter(self) -> IntoIter<&'static str, 10> {
        ["cookie"; 10].into_iter()
    }
}

fn main() {
    for cookie in CookieJar {
        println!("*munches {cookie}*");
    }
}

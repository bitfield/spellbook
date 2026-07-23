use std::array::IntoIter;

struct CookieJar;

impl IntoIterator for CookieJar {
    type IntoIter = IntoIter<Self::Item, 3>;
    type Item = &'static str;

    fn into_iter(self) -> IntoIter<&'static str, 3> {
        ["cookie"; 3].into_iter()
    }
}

fn main() {
    for cookie in CookieJar {
        println!("*munches {cookie}*");
    }
    // *munches cookie*
    // *munches cookie*
    // *munches cookie*
}

#[derive(Debug, PartialEq)]
pub struct Sausage;

fn main() {
    let soss1 = Sausage;
    let soss2 = Sausage;
    assert_eq!(soss1, soss2, "unequal sausages");
}

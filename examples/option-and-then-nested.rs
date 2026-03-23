#[derive(Debug)]
struct Ham;
type Filling = Option<Ham>;
type Sandwich = Option<Filling>;
type Lunchbox = Option<Sandwich>;

fn main() {
    let lunchbox = Lunchbox::from(Sandwich::from(Filling::from(Ham)));
    let filling = lunchbox
        .and_then(|sandwich| {
            println!("Time for lunch");
            sandwich
        })
        .and_then(|filling| {
            println!("Mmm. {filling:?}.");
            filling
        });
    println!("That was a nice bit of {filling:?}");
    // Time for lunch
    // Mmm. Some(Ham).
    // That was a nice bit of Some(Ham)
}

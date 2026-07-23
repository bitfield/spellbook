#[derive(Copy, Clone, Debug)]
pub struct Beer {
    pub strength: u8,
}

fn main() {
    let my_beer = Beer { strength: 5 };
    let your_beer = my_beer;
    println!("My beer: {my_beer:?}");
    println!("Your beer: {your_beer:?}");
    println!("Cheers, my dear!");
}

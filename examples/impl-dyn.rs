use std::fmt::Debug;

pub trait Eat: Debug {
    fn eat(&self) {
        println!("Mmm. {self:?}.");
    }
}

impl dyn Eat {
    pub fn yum(&self) {
        println!("Yum yum!");
    }
}

#[derive(Debug)]
pub struct Cake;

impl Eat for Cake {}

#[derive(Debug)]
pub struct Candy;

impl Eat for Candy {}

fn main() {
    let edibles: Vec<Box<dyn Eat>> = vec![Box::new(Cake), Box::new(Candy)];
    for snack in edibles {
        snack.eat();
        snack.yum();
    }
    // Mmm. Cake.
    // Yum yum!
    // Mmm. Candy.
    // Yum yum!
}

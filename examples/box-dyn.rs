use std::fmt::Debug;

pub trait Eat: Debug {
    fn eat(&self) {
        println!("Mmm. {self:?}.");
    }
}

#[derive(Debug)]
pub struct Cake;

impl Eat for Cake {}

impl dyn Eat {
    pub fn yum(&self) {
        println!("Yum yum!");
    }
}

#[derive(Debug)]
pub struct Candy;

impl Eat for Candy {}

fn main() {
    Cake.eat();
    // Mmm. Cake.
    Candy.eat();
    // Mmm. Candy.
    
    let edibles: Vec<Box<dyn Eat>> = vec![Box::new(Cake), Box::new(Candy)];
    for snack in edibles {
        snack.yum();
        snack.eat();
    }
    // Mmm. Cake.
    // Mmm. Candy.
}

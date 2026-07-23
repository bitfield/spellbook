use std::{any::Any, fmt::Debug};

pub trait Eat: Any + Debug {
    fn eat(&self) {
        println!("Mmm. {self:?}.");
    }
}

#[derive(Debug)]
pub struct Cake;

impl Eat for Cake {}

#[derive(Debug)]
pub struct Candy;

impl Eat for Candy {}

#[expect(clippy::shadow_unrelated, reason = "different approach")]
fn main() {
    let edibles: Vec<Box<dyn Eat>> = vec![Box::new(Cake), Box::new(Candy)];
    for snack in edibles {
        let snack_any: Box<dyn Any> = snack;
        if snack_any.is::<Cake>() {
            println!("Cake");
        } else {
            println!("Non-cake");
        }
    }
    // Cake
    // Non-cake

    let edibles: Vec<Box<dyn Eat>> = vec![Box::new(Cake), Box::new(Candy)];
    for snack in edibles {
        let snack_any: Box<dyn Any> = snack;
        if let Some(cake) = snack_any.downcast_ref::<Cake>() {
            cake.eat();
        }
    }
    // Mmm. Cake.
}

use crate::products::{Alcohol, Fruit, Book};

pub trait Visitor {
    fn visit_alcohol(&self, vodka: &Alcohol) -> f64;
    fn visit_fruit(&self, fruit: &Fruit) -> f64;
    fn visit_book(&self, book: &Book) -> f64;
}

pub struct TaxVisitor;

impl Visitor for TaxVisitor {
    fn visit_alcohol(&self, alcohol: &Alcohol) -> f64 {
        alcohol.price + alcohol.price*0.25 // high tax
    }

    fn visit_fruit(&self, fruit: &Fruit) -> f64 {
        fruit.price + fruit.price*0.05 // low tax
    }

    fn visit_book(&self, book: &Book) -> f64 {
        book.price + book.price*0.10 // moderate tax
    }
}
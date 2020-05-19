use crate::visitable::Visitable;
use crate::visitor::Visitor;

pub struct Alcohol {
    pub price: f64,
}

impl Visitable for Alcohol {
    fn accept(&self, visitor: Box<&dyn Visitor>) -> f64 {
        visitor.visit_alcohol(&self)
    }
}

pub struct Fruit {
    pub price: f64,
}

impl Visitable for Fruit {
    fn accept(&self, visitor: Box<&dyn Visitor>) -> f64 {
        visitor.visit_fruit(&self)
    }
}

pub struct Book {
    pub price: f64,
}

impl Visitable for Book {
    fn accept(&self, visitor: Box<&dyn Visitor>) -> f64 {
        visitor.visit_book(&self)
    }
}
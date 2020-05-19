use crate::visitor::Visitor;

pub trait Visitable {
    fn accept(&self, visitor: Box<&dyn Visitor>) -> f64;
}
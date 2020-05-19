mod visitor;
mod visitable;
mod products;

use products::{Alcohol, Fruit, Book};
use visitor::{Visitor, TaxVisitor};

pub fn test() {
    let tax_visitor = TaxVisitor;
    
    let vodka = Alcohol {price: 50.0};
    let mango = Fruit{price: 4.5};
    let harry_potter = Book{price: 45.0};

    println!("vodka price with taxes: {} $", tax_visitor.visit_alcohol(&vodka));
    println!("mango price with taxes: {} $", tax_visitor.visit_fruit(&mango));
    println!("Harry Potter price with taxes: {} $", tax_visitor.visit_book(&harry_potter));
}
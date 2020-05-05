use crate::Stock;

pub trait Command {
    fn execute(&self);
}

pub struct BuyStock<'a> {
    stock: &'a Stock,
}

impl<'a> BuyStock<'a> {
    pub fn new(stock: &'a Stock) -> BuyStock<'a> {
        BuyStock{stock}
    }
}

impl<'a> Command for BuyStock<'a> {
    fn execute(&self) {
        self.stock.buy();
    }
}

pub struct SellStock<'a> {
    stock: &'a Stock,
}

impl<'a> SellStock<'a> {
    pub fn new(stock: &'a Stock) -> SellStock<'a> {
        SellStock{stock}
    }
}

impl<'a> Command for SellStock<'a> {
    fn execute(&self) {
        self.stock.sell();
    }
}
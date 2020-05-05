
pub struct Stock {
    name: String,
    quantity: u32,
}

impl Stock {
    pub fn new(name: String, quantity: u32) -> Stock {
        Stock{name, quantity}
    }

    pub fn buy(&self) {
        println!("Buying stock: {}, quantity: {}", self.name, self.quantity);
    }

    pub fn sell(&self) {
        println!("Selling stock: {}, quantity: {}", self.name, self.quantity);
    }
}
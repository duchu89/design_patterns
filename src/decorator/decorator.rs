use crate::decorator::pizza::Pizza;

pub struct Cheese {
    pub pizza: Box<dyn Pizza>,
}

impl Pizza for Cheese {
    fn get_description(&self) -> String {
        self.pizza.get_description() + &" with Cheese".to_string() 
    }

    fn get_price(&self) -> f64 {
        self.pizza.get_price() + 6.0
    }
}

pub struct Chicken {
    pub pizza: Box<dyn Pizza>,
}

impl Pizza for Chicken {
    fn get_description(&self) -> String {
        self.pizza.get_description() + &" with Chicken".to_string() 
    }

    fn get_price(&self) -> f64 {
        self.pizza.get_price() + 12.0
    }
}

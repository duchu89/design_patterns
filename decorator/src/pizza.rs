pub trait Pizza {
    fn get_description(&self) -> String;
    fn get_price(&self) -> f64;
}

pub struct Margherita;

impl Pizza for Margherita {
    fn get_description(&self) -> String {
        "Margherita".to_string()
    }

    fn get_price(&self) -> f64 {
        64.0
    }
}

pub struct Vegetariana;

impl Pizza for Vegetariana {
    fn get_description(&self) -> String {
        "Vegetariana".to_string()
    }

    fn get_price(&self) -> f64 {
        75.0
    }
}
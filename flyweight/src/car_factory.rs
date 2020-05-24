use crate::car::{Car};
use std::collections::HashMap;
use std::rc::Rc;

pub struct CarFactory {
    car_map: HashMap<String, Rc<Car>>,
}

impl CarFactory{
    pub fn new() -> CarFactory {
        CarFactory {
            car_map: HashMap::new(),
        }
    }

    pub fn spawn_car(&mut self, color: String) -> Rc<Car> {
        if let Some(car) = self.car_map.get(&color) {
            Rc::clone(car)
        } else {
            println!("Creating new car of color: {}", color);
            let car = Rc::new(Car{color: color.clone()});
            let return_val = Rc::clone(&car);
            self.car_map.insert(color, car);
            return_val
        }
    }
}


pub trait ElectricCar {
    fn turn_on(&self);
    fn turn_off(&self);
    fn accelerate(&self);
}

pub struct TeslaCar {
    pub name: String,
}

impl ElectricCar for TeslaCar {
    fn turn_on(&self) {
        println!("Turning on Tesla {}", self.name);
    }

    fn turn_off(&self) {
        println!("Turning off Tesla {}", self.name);
    }

    fn accelerate(&self) {
        println!("Accelerating Tesla {}", self.name);
    }
}
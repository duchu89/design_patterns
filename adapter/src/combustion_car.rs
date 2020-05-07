pub trait CombustionCar {
    fn turn_on(&self);
    fn turn_off(&self);
    fn ignite_engine(&self);
    fn choose_gear(&self);
    fn accelerate(&self);
}

pub struct SubaruCar {
    pub name: String,
}

impl CombustionCar for SubaruCar {
    fn turn_on(&self) {
        println!("Turning on Subaru {}", self.name);
    }

    fn turn_off(&self) {
        println!("Turning off Subaru {}", self.name);
    }

    fn ignite_engine(&self) {
        println!("Igniting engine of Subaru {}", self.name);
    }

    fn choose_gear(&self) {
        println!("Choosing gear on Subaru {}", self.name);
    }
    fn accelerate(&self) {
        println!("Accelerating Subaru {}", self.name);
    }
}
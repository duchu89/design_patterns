use crate::combustion_car::CombustionCar;
use crate::electric_car::ElectricCar;

pub struct ElectricCarAdapter {
    pub electric_car: Box<dyn ElectricCar>,
}

impl CombustionCar for ElectricCarAdapter {
    fn turn_on(&self) {
        self.electric_car.turn_on();
    }

    fn turn_off(&self) {
        self.electric_car.turn_off();
    }

    fn ignite_engine(&self) {
        //DO NOTHING - electric car doesn't require ignition
    }

    fn choose_gear(&self) {
        //DO NOTHING - electric car doesn't have a gearbox
    }

    fn accelerate(&self) {
        self.electric_car.accelerate();
    }
}
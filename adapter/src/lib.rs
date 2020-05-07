mod adapter;
mod combustion_car;
mod electric_car;

use adapter::ElectricCarAdapter;
use combustion_car::{CombustionCar, SubaruCar};
use electric_car::TeslaCar;

pub fn test() {
    let subaru_impreza = SubaruCar {
        name: "Impreza".to_string(),
    };
    drive(&subaru_impreza);

    let tesla_model_s = TeslaCar {
        name: "Model S".to_string(),
    };

    let tesla_adapter = ElectricCarAdapter {
        electric_car: Box::new(tesla_model_s),
    };
    drive(&tesla_adapter);
}

fn drive<T: CombustionCar>(car: &T) {
    car.turn_on();
    car.ignite_engine();
    car.choose_gear();
    car.accelerate();
    car.turn_off();
    println!("\n");
}

mod car;
mod car_factory;

use car::Vehicle;
use car_factory::CarFactory;

use rand::Rng;

static COLORS: [&'static str; 6] = ["Green", "Red", "Blue", "Yellow", "Black", "White"];

pub fn test() {
    let mut car_factory = CarFactory::new();
    
    for number in 0..10 {
        let color = get_random_color();
        let car = car_factory.spawn_car(color);
        car.draw(number*get_random_x(), number*get_random_y());
    }
}

fn get_random_color() -> String {
    let random_index = rand::thread_rng().gen_range(0, COLORS.len()-1);
    COLORS[random_index].to_string()
}

fn get_random_x() -> i32 {
    rand::thread_rng().gen_range(0, 100)
}

fn get_random_y() -> i32 {
    rand::thread_rng().gen_range(0, 50)
}
mod application;
mod method;

use application::Application;
use method::{WoodenHouse, ConcreteHouse};

pub fn test() {
    let wooden_house = WoodenHouse;
    let first_application = Application::new(wooden_house);
    first_application.build_house();

    let concrete_house = ConcreteHouse;
    let second_application = Application::new(concrete_house);
    second_application.build_house();
}
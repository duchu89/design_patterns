use crate::method::House;

pub struct Application<T: House> {
    house: T,
}

impl<T: House> Application<T> {
    pub fn new(house: T) -> Application<T> {
        Application { house }
    }

    pub fn build_house(&self) {
        self.build_foundation();
        self.house.build_pillars();
        self.house.build_walls();
        self.build_windows();
        println!("House is built")
    }

    fn build_foundation(&self) {
        println!("Building foundation");
    }

    fn build_windows(&self) {
        println!("Building windows");
    }
}

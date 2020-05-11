
pub trait House {
    fn build_walls(&self);
    fn build_pillars(&self);
}

pub struct WoodenHouse;

impl House for WoodenHouse {
    fn build_walls(&self) {
        println!("Building wooden walls");
    }

    fn build_pillars(&self) {
        println!("Building wooden pillars");
    }
}

pub struct ConcreteHouse;

impl House for ConcreteHouse {
    fn build_walls(&self) {
        println!("Building concrete walls");
    }

    fn build_pillars(&self) {
        println!("Building concrete pillars");
    }
}

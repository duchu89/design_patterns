pub trait Vehicle {
    fn draw(&self, position_x: i32, position_y: i32);
}

pub struct Car {
    pub color: String,
}

impl Vehicle for Car {
    fn draw(&self, position_x: i32, position_y: i32) {
        println!("Drawing car, color: {}, position x: {}, position y: {}", self.color, position_x, position_y);
    }
}
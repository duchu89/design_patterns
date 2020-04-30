pub trait Button {
    fn draw(&self);
}

pub struct MaterialButton;

impl Button for MaterialButton {
    fn draw(&self) {
        println!("Drawing Material style Button");
    }
}

pub struct CupertinoButton;

impl Button for CupertinoButton {
    fn draw(&self) {
        println!("Drawing Cupertino style Button");
    }
}
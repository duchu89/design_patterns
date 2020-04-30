pub trait ProgressBar {
    fn draw(&self);
}

pub struct MaterialProgressBar;

impl ProgressBar for MaterialProgressBar {
    fn draw(&self) {
        println!("Drawing Material style Progress Bar");
    }
}

pub struct CupertinoProgressBar;

impl ProgressBar for CupertinoProgressBar {
    fn draw(&self) {
        println!("Drawing Cupertino style Progress Bar");
    }
}
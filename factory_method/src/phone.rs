pub trait Phone {
    fn desc(&self);
}

pub struct IPhone11Max;

impl Phone for IPhone11Max {
    fn desc(&self) {
        println!("I'm a new iPhone XS");
    }
}

pub struct IPhoneSE;

impl Phone for IPhoneSE {
    fn desc(&self) {
        println!("I'm a new iPhone SE");
    }
}

pub struct Pixel4XL;

impl Phone for Pixel4XL {
    fn desc(&self) {
        println!("I'm a new Pixel 4 XL");
    }
}

pub struct Pixel4a;

impl Phone for Pixel4a {
    fn desc(&self) {
        println!("I'm a new Pixel 4a");
    }
}

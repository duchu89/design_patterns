use crate::factory_method::phone::{Phone, IPhone11Max, IPhoneSE, Pixel4XL,Pixel4a};
use crate::factory_method::specification::Specification;

pub trait Factory {
    fn new_phone(&self, spec: Specification) -> Box<dyn Phone>;
}

pub struct AppleFactory;

impl Factory for AppleFactory {
    fn new_phone(&self, spec: Specification) -> Box<dyn Phone> {
        match spec {
            Specification::SMALL => Box::new(IPhoneSE),
            Specification::BIG => Box::new(IPhone11Max),
        }
    }
}

pub struct GoogleFactory;

impl Factory for GoogleFactory {
    fn new_phone(&self, spec: Specification) -> Box<dyn Phone> {
        match spec {
            Specification::SMALL => Box::new(Pixel4a),
            Specification::BIG => Box::new(Pixel4XL),
        }
    }
}
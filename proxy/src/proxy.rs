pub trait ExpensiveCalculation {
    fn process(&mut self);
}

pub struct ExpensiveObject;

impl ExpensiveObject {
    pub fn new() -> ExpensiveObject {
        ExpensiveObject::heavy_initial_configuration();
        ExpensiveObject
    }

    fn heavy_initial_configuration() {
        println!("Loading initial configuration...");
    }
}

impl ExpensiveCalculation for ExpensiveObject {
    fn process(&mut self) {
        println!("processing complete.");
    }
}

pub struct ExpensiveObjectProxy {
    object: Option<ExpensiveObject>,
}

impl ExpensiveObjectProxy {
    pub fn new() -> ExpensiveObjectProxy {
        ExpensiveObjectProxy { object: None }
    }
}

impl ExpensiveCalculation for ExpensiveObjectProxy {
    fn process(&mut self) {
        match &mut self.object {
            Some(v) => v.process(),
            None => {
                self.object = Some(ExpensiveObject::new());
                self.object.as_mut().unwrap().process();
            }
        }
    }
}

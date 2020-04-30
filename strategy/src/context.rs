use crate::strategy::Strategy;

pub struct Context {
    pub strategy: Box<dyn Strategy>,
}

impl Context {
    pub fn arrange(&self, input: &[i32]) {
        self.strategy.sort(input);
    }
}

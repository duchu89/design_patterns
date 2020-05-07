mod proxy;

use proxy::{ExpensiveCalculation, ExpensiveObjectProxy};

pub fn test() {
    let mut expensive_object: Box<dyn ExpensiveCalculation> =
        Box::new(ExpensiveObjectProxy::new()) as Box<dyn ExpensiveCalculation>;
        
    expensive_object.process();
    expensive_object.process();
}

#[path = "strategy/test_strategy.rs"]
mod strategy;

#[path = "observer/test_observer.rs"]
mod observer;

#[path = "decorator/test_decorator.rs"]
mod decorator;

fn main() {
    println!("#### STRATEGY ####");
    strategy::test();
    println!("#### END STRATEGY ####");

    println!("#### OBSERVER ####");
    observer::test();
    println!("#### END OBSERVER ####");
    
    println!("#### DECORATOR ####");
    decorator::test();
    println!("#### END DECORATOR ####");
}

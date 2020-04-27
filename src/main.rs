#[path = "strategy/test_strategy.rs"]
mod strategy;

#[path = "observer/test_observer.rs"]
mod observer;

#[path = "decorator/test_decorator.rs"]
mod decorator;

#[path = "factory_method/test_factory_method.rs"]
mod factory_method;

#[path = "abstract_factory/test_abstract_factory.rs"]
mod abstract_factory;

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

    println!("#### FACTORY METHOD ####");
    factory_method::test();
    println!("#### END FACTORY METHOD ####");

    println!("#### ABSTRACT FACTORY METHOD ####");
    abstract_factory::test();
    println!("#### END ABSTRACT FACTORY METHOD ####");
}

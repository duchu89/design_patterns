use strategy;
use observer;
use decorator;
use factory_method;
use abstract_factory;
use command;
use adapter;
use proxy;
use bridge;
use template_method;
use composit;
use visitor;

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

    println!("#### COMMAND ####");
    command::test();
    println!("#### END COMMAND ####");

    println!("#### ADAPTER ####");
    adapter::test();
    println!("#### END ADAPTER ####");

    println!("#### PROXY ####");
    proxy::test();
    println!("#### END PROXY ####");

    println!("#### BRIDGE ####");
    bridge::test();
    println!("#### END BRIDGE ####");

    println!("#### TEMPLATE METHOD ####");
    template_method::test();
    println!("#### END TEMPLATE METHOD ####");

    println!("#### COMPOSIT ####");
    composit::test();
    println!("#### END COMPOSIT ####");

    println!("#### VISITOR ####");
    visitor::test();
    println!("#### END VISITOR ####");
}

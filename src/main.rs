#[path = "strategy/test_strategy.rs"]
mod strategy;

#[path = "observer/test_observer.rs"]
mod observer;

fn main() {
    strategy::test();
    observer::test();
}

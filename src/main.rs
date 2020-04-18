#[path = "strategy/bubble_sort.rs"]
mod bubble_sort;
#[path = "strategy/context.rs"]
mod context;
#[path = "strategy/quick_sort.rs"]
mod quick_sort;
#[path = "strategy/strategy.rs"]
mod strategy;

use bubble_sort::BubbleSort;
use context::Context;
use quick_sort::QuickSort;

fn main() {
    let numbers: [i32; 5] = [3, 1, 5, 7, 0];

    let mut context = Context {
        strategy: Box::new(BubbleSort),
    };

    context.arrange(&numbers);

    context = Context {
        strategy: Box::new(QuickSort),
    };

    context.arrange(&numbers);
}

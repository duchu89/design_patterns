mod bubble_sort;
mod context;
mod quick_sort;
mod strategy;

use bubble_sort::BubbleSort;
use context::Context;
use quick_sort::QuickSort;

pub fn test() {
    let numbers: [i32; 5] = [3, 1, 5, 7, 0];

    let mut context = Context {
        strategy: Box::new(BubbleSort),
    };

    context.arrange(&numbers);


    context.strategy = Box::new(QuickSort);

    context = Context {
        strategy: Box::new(QuickSort),
    };

    context.arrange(&numbers);
}

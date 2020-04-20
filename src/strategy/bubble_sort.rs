use crate::strategy::strategy::Strategy;

pub struct BubbleSort;

impl Strategy for BubbleSort {
    fn sort(&self, arr: &[i32]) {
        println!("sorting array: {:?} using Bubble Sort alghoritm", arr);
    }
}

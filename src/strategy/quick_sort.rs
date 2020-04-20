use crate::strategy::strategy::Strategy;

pub struct QuickSort;

impl Strategy for QuickSort {
    fn sort(&self, arr: &[i32]) {
        println!("sorting array: {:?} using Quick Sort alghoritm", arr);
    }
}

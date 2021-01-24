use std::fmt::Debug;
use std::fmt::Display;

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Display + Debug;
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord + Display + Debug,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}

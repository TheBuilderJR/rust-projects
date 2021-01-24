use super::Sorter;
use std::fmt::Debug;
use std::fmt::Display;

pub struct QuickSort;

fn quicksort<T: Ord + Display + Debug>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice not empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    if left > 0 {
        slice.swap(left, 0)
    }

    if slice[..left].len() > 0 {
        quicksort(&mut slice[..left]);
    }
    if slice[(left + 1)..].len() > 0 {
        quicksort(&mut slice[(left + 1)..]);
    }
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Display + Debug,
    {
        // [ unsorted, pivot, unsorted ]
        quicksort(slice)
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

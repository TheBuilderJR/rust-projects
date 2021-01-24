use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted

            if self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i, i - 1);
                    i -= 1;
                }
            } else {
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}

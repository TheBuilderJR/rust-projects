use sorts::*;

use rand::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: Display> fmt::Display for SortEvaluator<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.t)
    }
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}
impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("T: Ord")
    }
}

fn bench<T: Ord + Clone + Debug + Display, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> usize {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    counter.get()
}

fn main() {
    let counter = Rc::new(Cell::new(0));
    let mut rng = rand::thread_rng();
    for &n in &[0, 1, 10, 100, 1000, 10000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rng.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rng);

            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {}", "Bubble", n, took);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("{} {} {}", "Insertion Smart", n, took);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("{} {} {}", "Insertion Dumb", n, took);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {}", "Selection", n, took);
            let took = bench(QuickSort, &values, &counter);
            println!("{} {} {}", "Quick", n, took);
        }
    }
}

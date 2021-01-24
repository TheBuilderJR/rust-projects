use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by unsafe cell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T where T: Copy {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    user super::Cell;

    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = std::sync::Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        thread::spawn(|| {
            x.set(43);
        })
        let x2 = Arc::clone(&x);
        thread::spawn(|| {
            x2.set(44);
        })
    }

    #[test]
    fn bad2() {
        let x = Cell::new(vec![42]);
        let first = &x.get()[0];
        x.set(vec![]);
    }
}

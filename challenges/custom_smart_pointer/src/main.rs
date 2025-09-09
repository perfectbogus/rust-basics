use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::{Deref, DerefMut};

static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let box1 = TrackedBox::new(42);
    let box2 = TrackedBox::new("hello");

    println!("Value: {}", *box1);

    // println!("Count: {}", TrackedBox::allocation_count())
}

struct TrackedBox<T> {
    inner: Box<T>,
    id: usize,
}

impl<T> TrackedBox<T> {
    fn new(value: T) -> Self {
        let count = ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
        TrackedBox { inner: Box::new(value), id: count }
    }

    fn allocation_count() -> usize {
        ALLOCATION_COUNT.load(Ordering::SeqCst)
    }
}

impl<T> Deref for TrackedBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<T> DerefMut for TrackedBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}

impl<T> Drop for TrackedBox<T> {
    fn drop(&mut self) {
        ALLOCATION_COUNT.fetch_sub(1, Ordering::SeqCst);
        println!("Dropping TrackedBox with id: {}", self.id);
    }
}



#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    // TODO: your code goes here.
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn push(&mut self, val: T) {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn front(&self) -> Option<&T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn min(&self) -> Option<&T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
        unimplemented!()
    }
}

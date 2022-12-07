#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    data: VecDeque<T>,
    minimums: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            minimums: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push_back(val.clone());
        loop {
            let last_min = self.minimums.back();
            match last_min {
                Some(minimum) if minimum > &val => {
                    self.minimums.pop_back();
                }
                Some(..) | None => {
                    break;
                }
            }
        }
        self.minimums.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.front() == self.minimums.front() {
            self.minimums.pop_front();
        }
        self.data.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.minimums.front()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

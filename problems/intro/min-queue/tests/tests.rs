use min_queue::MinQueue;
use ntest::timeout;
use rand::Rng;
use std::collections::VecDeque;

struct NaiveMinQueue<T> {
    data: VecDeque<T>,
}

impl<T: Clone + Ord> NaiveMinQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.data.iter().min()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[test]
fn it_works() {
    let mut queue = MinQueue::new();
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.is_empty(), true);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.min(), None);

    assert_eq!(queue.pop(), None);
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.is_empty(), true);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.min(), None);

    queue.push(2);
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &2);
    assert_eq!(queue.min().unwrap(), &2);

    queue.push(3);
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &2);
    assert_eq!(queue.min().unwrap(), &2);

    queue.push(4);
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &2);
    assert_eq!(queue.min().unwrap(), &2);

    queue.push(1);
    assert_eq!(queue.len(), 4);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &2);
    assert_eq!(queue.min().unwrap(), &1);

    queue.push(5);
    assert_eq!(queue.len(), 5);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &2);
    assert_eq!(queue.min().unwrap(), &1);

    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.len(), 4);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &3);
    assert_eq!(queue.min().unwrap(), &1);

    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &4);
    assert_eq!(queue.min().unwrap(), &1);

    assert_eq!(queue.pop(), Some(4));
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &1);
    assert_eq!(queue.min().unwrap(), &1);

    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.front().unwrap(), &5);
    assert_eq!(queue.min().unwrap(), &5);

    assert_eq!(queue.pop(), Some(5));
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.is_empty(), true);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.min(), None);

    assert_eq!(queue.pop(), None);
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.is_empty(), true);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.min(), None);
}

#[test]
fn compare_with_naive() {
    let mut queue = MinQueue::new();
    let mut naive = NaiveMinQueue::new();
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        if rng.gen_range(0..3) == 0 {
            assert_eq!(queue.pop(), naive.pop());
        } else {
            let value = rng.gen::<u64>();
            queue.push(value);
            naive.push(value);
        }
        assert_eq!(queue.len(), naive.len());
        assert_eq!(queue.is_empty(), naive.is_empty());
        assert_eq!(queue.front(), naive.front());
        assert_eq!(queue.min(), naive.min());
    }
}

#[test]
#[timeout(2000)]
fn stress() {
    let mut queue = MinQueue::new();
    let mut rng = rand::thread_rng();
    for _ in 0..300000 {
        match rng.gen_range(0..4) {
            0 => {
                queue.pop();
            }
            1 => {
                queue.min();
            }
            _ => {
                queue.push(rng.gen::<u64>());
            }
        }
    }
}

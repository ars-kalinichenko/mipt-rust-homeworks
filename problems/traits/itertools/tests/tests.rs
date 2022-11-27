use itertools::ExtendedIterator;
use std::{cell::Cell, collections::HashSet, rc::Rc};

////////////////////////////////////////////////////////////////////////////////

struct Clonable {
    payload: usize,
    cloned_count: Rc<Cell<usize>>,
}

impl Clonable {
    fn new(payload: usize) -> Self {
        Self {
            payload,
            cloned_count: Rc::new(Cell::new(0)),
        }
    }

    fn cloned_count(&self) -> usize {
        self.cloned_count.get()
    }
}

impl Clone for Clonable {
    fn clone(&self) -> Self {
        self.cloned_count.set(self.cloned_count.get() + 1);
        Self {
            payload: self.payload,
            cloned_count: self.cloned_count.clone(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

struct TrackedIter<I> {
    inner: I,
    advanced_count: Rc<Cell<usize>>,
}

impl<I> TrackedIter<I> {
    fn new<T: IntoIterator<IntoIter = I>>(into_iter: T) -> (Self, Rc<Cell<usize>>) {
        let advanced_count = Rc::new(Cell::new(0));
        let iter = Self {
            inner: into_iter.into_iter(),
            advanced_count: advanced_count.clone(),
        };
        (iter, advanced_count)
    }
}

impl<I: Iterator> Iterator for TrackedIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.advanced_count.set(self.advanced_count.get() + 1);
        self.inner.next()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct SkipIter<I: Iterator> {
    iter: I,
    advanced_count: usize,
    to_skip: HashSet<usize>,
}

impl<I: Iterator> SkipIter<I> {
    fn new(iter: I, to_skip: HashSet<usize>) -> Self {
        Self {
            iter,
            advanced_count: 0,
            to_skip,
        }
    }
}

impl<I: Iterator> Iterator for SkipIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.advanced_count += 1;
        if self.to_skip.contains(&(self.advanced_count - 1)) {
            None
        } else {
            self.iter.next()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn lazy_cycle_simple() {
    let elems = vec![Clonable::new(0), Clonable::new(1)];
    let (tracked_iter, advanced_count) = TrackedIter::new(elems.clone());

    let mut iter = tracked_iter.lazy_cycle();
    assert_eq!(advanced_count.get(), 0);
    assert_eq!(elems[0].cloned_count(), 1);
    assert_eq!(elems[1].cloned_count(), 1);

    let first = iter.next().unwrap();
    assert_eq!(advanced_count.get(), 1);
    assert_eq!(first.payload, 0);
    assert_eq!(elems[0].cloned_count(), 2);
    assert_eq!(elems[1].cloned_count(), 1);

    let second = iter.next().unwrap();
    assert_eq!(advanced_count.get(), 2);
    assert_eq!(second.payload, 1);
    assert_eq!(elems[0].cloned_count(), 2);
    assert_eq!(elems[1].cloned_count(), 2);

    let third = iter.next().unwrap();
    assert_eq!(advanced_count.get(), 3);
    assert_eq!(third.payload, 0);
    assert_eq!(elems[0].cloned_count(), 3);
    assert_eq!(elems[1].cloned_count(), 2);

    for i in 0..1000 {
        let elem = iter.next().unwrap();
        assert_eq!(advanced_count.get(), 3);
        assert_eq!(elem.payload, (i + 1) % 2);
        assert_eq!(elems[0].cloned_count(), 3 + (i + 1) / 2);
        assert_eq!(elems[1].cloned_count(), 3 + i / 2);
    }
}

#[test]
fn lazy_cycle_skip() {
    let iter = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter();
    let to_skip = [3, 4, 5, 7, 8].into_iter().collect();
    let mut cycle = SkipIter::new(iter, to_skip).lazy_cycle();
    for i in 0..10 {
        let elem = cycle.next();
        assert_eq!(elem, Some(i % 3));
    }
}

#[test]
fn lazy_cycle_empty() {
    let mut iter = Vec::<i32>::new().into_iter().lazy_cycle();
    assert!(iter.next().is_none());
}

#[test]
fn extract_simple() {
    struct Int(usize);
    let mut iter: Box<dyn ExtendedIterator<Item = Int>> = Box::new((0..100).map(Int));

    for i in (1..100).step_by(2) {
        let (mb_extracted, new_iter) = iter.extract(1 + i / 2);
        assert_eq!(mb_extracted.unwrap().0, i);
        iter = Box::new(new_iter);
    }

    assert_eq!(
        iter.map(|x| x.0).collect::<Vec<_>>(),
        (0..100).step_by(2).collect::<Vec<_>>()
    );
}

#[test]
fn extract_from_extract() {
    let (mb_extracted1, iter) = [0, 1, 2, 3, 4, 5, 6].into_iter().extract(4);
    let (mb_extracted2, iter) = iter.extract(5);
    let (mb_extracted3, iter) = iter.extract(3);
    assert_eq!(mb_extracted1, Some(4));
    assert_eq!(mb_extracted2, Some(6));
    assert_eq!(mb_extracted3, Some(3));
    assert_eq!(iter.collect::<Vec<_>>(), vec![0, 1, 2, 5]);
}

#[test]
fn extract_nonexistent() {
    let (mb_extracted, iter) = [0, 1, 2].into_iter().extract(10);
    assert!(mb_extracted.is_none());
    assert_eq!(iter.collect::<Vec<_>>(), vec![0, 1, 2]);
}

#[test]
fn tee_simple() {
    let elems = (0..5).map(Clonable::new).collect::<Vec<_>>();
    let (tracked_iter, advanced_count) = TrackedIter::new(elems.clone());

    let (mut first, mut second) = tracked_iter.tee();
    assert_eq!(advanced_count.get(), 0);
    assert!(elems.iter().all(|e| e.cloned_count() == 1));

    for i in 0..2 {
        assert_eq!(first.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), i + 1);
        assert!(elems[..i + 1].iter().all(|e| e.cloned_count() == 2));
        assert!(elems[i + 1..].iter().all(|e| e.cloned_count() == 1));
    }

    for i in 0..2 {
        assert_eq!(second.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), 2);
        assert!(elems[..2].iter().all(|e| e.cloned_count() == 2));
        assert!(elems[2..].iter().all(|e| e.cloned_count() == 1));
    }

    for i in 2..5 {
        assert_eq!(second.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), i + 1);
        assert!(elems[..i + 1].iter().all(|e| e.cloned_count() == 2));
        assert!(elems[i + 1..].iter().all(|e| e.cloned_count() == 1));
    }

    for _ in 0..3 {
        assert!(second.next().is_none());
        assert_eq!(advanced_count.get(), 6);
        assert!(elems.iter().all(|e| e.cloned_count() == 2));
    }

    for i in 2..5 {
        assert_eq!(first.next().unwrap().payload, i);
        assert_eq!(advanced_count.get(), 6);
        assert!(elems.iter().all(|e| e.cloned_count() == 2));
    }

    for _ in 0..3 {
        assert!(first.next().is_none());
        assert_eq!(advanced_count.get(), 6);
        assert!(elems.iter().all(|e| e.cloned_count() == 2));
    }
}

#[test]
fn tee_empty() {
    let vec: Vec<i32> = vec![];
    let (tracked_iter, advanced_count) = TrackedIter::new(vec);
    let (mut first, mut second) = tracked_iter.tee();

    for _ in 0..5 {
        assert!(first.next().is_none());
        assert_eq!(advanced_count.get(), 1);

        assert!(second.next().is_none());
        assert_eq!(advanced_count.get(), 1);
    }
}

#[test]
fn group_by_simple() {
    #[derive(Debug)]
    struct Int(usize);

    #[derive(PartialEq, Eq, Debug)]
    struct IsOdd(bool);

    let vec = [2, 4, 6, 1, 3, 5, 8, 10, 1, 2]
        .into_iter()
        .map(Int)
        .collect::<Vec<_>>();

    let mut num_called = 0;
    let mut iter = vec.into_iter().group_by(|x| {
        num_called += 1;
        IsOdd(x.0 % 2 == 1)
    });

    let expected_groups = vec![
        (false, vec![2, 4, 6]),
        (true, vec![1, 3, 5]),
        (false, vec![8, 10]),
        (true, vec![1]),
        (false, vec![2]),
    ];
    for (expected_odd, expected_group) in expected_groups.into_iter() {
        let (is_odd, group) = iter.next().unwrap();
        assert_eq!(is_odd, IsOdd(expected_odd));
        assert_eq!(
            expected_group,
            group.into_iter().map(|x| x.0).collect::<Vec<_>>(),
        );
    }

    assert!(iter.next().is_none());
}

#[test]
fn group_by_empty() {
    assert!(Vec::<i32>::new()
        .into_iter()
        .group_by(|x| *x)
        .next()
        .is_none());
}

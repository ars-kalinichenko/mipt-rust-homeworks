use smol_vec::SmolVec;

use std::{
    mem::{size_of, size_of_val},
    rc::Rc,
};

#[test]
fn simple() {
    let mut vec = SmolVec::<i32, 2>::new();
    assert!(size_of_val(&vec) == 2 * size_of::<i32>() + size_of::<usize>());

    assert_eq!(vec.pop(), None);
    assert_eq!(vec.push(1), None);
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);

    assert_eq!(vec.push(10), None);
    assert_eq!(vec.push(25), None);
    assert_eq!(vec.push(45), Some(45));
    assert_eq!(vec[0], 10);
    assert_eq!(vec[1], 25);
    vec[1] = 350;
    assert_eq!(vec[1], 350);
    assert_eq!(vec.pop(), Some(350));
    assert_eq!(vec[0], 10);
    assert_eq!(vec.pop(), Some(10));
    assert_eq!(vec.pop(), None);
}

#[test]
#[should_panic]
fn out_of_bounds_panic() {
    let mut vec = SmolVec::<i32, 100>::new();
    vec.push(50);
    vec[1];
}

#[test]
#[should_panic]
fn out_of_bounds_mut_panic() {
    let mut vec = SmolVec::<i32, 0>::new();
    vec[0] = 34;
}

#[test]
fn drop_vec() {
    let obj = Rc::new(50);

    let mut vec = SmolVec::<_, 10>::new();
    for _ in 0..10 {
        vec.push(obj.clone());
    }

    assert_eq!(Rc::strong_count(&obj), 11);
    vec.pop();
    assert_eq!(Rc::strong_count(&obj), 10);
    drop(vec);
    assert_eq!(Rc::strong_count(&obj), 1);
}

use pstack::PStack;

#[test]
fn simple() {
    let mut stack = PStack::new();
    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());

    for i in 0..10 {
        stack = stack.push(i);
        assert_eq!(stack.len(), i + 1);
    }

    for i in (0..10).rev() {
        let (last, stack_new) = stack.pop().unwrap();
        assert_eq!(stack_new.len(), i);
        assert_eq!(*last, i);
        stack = stack_new;
    }
}

#[test]
fn persistence() {
    let mut stacks = vec![PStack::new()];
    for i in 0..100 {
        let st = stacks.last_mut().unwrap().push(i);
        stacks.push(st);
    }

    for i in (0..100).rev() {
        let (top, tail) = stacks.last().unwrap().pop().unwrap();
        assert_eq!(*top, i);
        stacks.push(tail);
    }

    for i in 0..100 {
        let stack = stacks[i].clone();
        assert_eq!(stack.len(), i);

        let mut cnt = 0;
        for (item, i) in stack.iter().zip((0..i).rev()) {
            assert_eq!(i, *item);
            cnt += 1;
        }
        assert_eq!(i, cnt);
        drop(stack);
    }

    for i in 100..201 {
        let stack = stacks[i].clone();
        assert_eq!(stack.len(), 200 - i);

        let mut cnt = 0;
        for (item, i) in stack.iter().zip((0..200 - i).rev()) {
            assert_eq!(i, *item);
            cnt += 1;
        }
        assert_eq!(200 - i, cnt);
    }
}

#[test]
fn no_clone() {
    struct Int(i32);

    let mut stack = PStack::new();
    for i in 0..100 {
        stack = stack.push(Int(i));
    }

    for i in (0..100).rev() {
        let (top, tail) = stack.pop().unwrap();
        assert_eq!(top.0, i);
        stack = tail;
    }
}

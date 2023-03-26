#![forbid(unsafe_code)]


use std::ops::Deref;
use std::rc::Rc;

pub struct PRef<T> {
    /*
    В данном случае PRef - это обертка над ссылкой на некоторый объект типа T.
    Она используется для того, чтобы предотвратить несколько изменяемых ссылок
    на один и тот же объект, что может привести к неопределенному поведению или ошибкам
     */
    value: Rc<T>,
}

impl<T> PRef<T> {
    pub fn new(value: T) -> Self {
        PRef {
            value: Rc::new(value),
        }
    }
}

impl<T> Clone for PRef<T> {
    fn clone(&self) -> Self {
        PRef {
            value: self.value.clone(),
        }
    }
}

impl<T> Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

type Link<T> = Option<PRef<Node<T>>>;

struct Node<T> {
    elem: PRef<T>,
    next: Link<T>,
}

pub struct PStack<T> {
    head: Link<T>,
    // последний элемент со ссылкой на стек из следующих
    length: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        PStack {
            head: None,
            length: 0,
        }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        PStack {
            head: self.head.clone(),
            length: self.length,
        }
    }
}


impl<T> PStack<T> {
    pub fn new() -> Self {
        PStack {
            head: None,
            length: 0,
        }
    }

    pub fn push(&self, value: T) -> Self {
        PStack {
            head: Some(PRef::new(Node { elem: PRef::new(value), next: self.head.clone() })),
            length: self.length + 1,
        }
    }


    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        let un = self.head.clone();
        match un {
            Some(node) => {
                Some((node.elem.clone(), PStack { head: node.next.clone(), length: self.length - 1 }))
            }
            None => None
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        Iter {
            next: self.head.clone()
        }
    }
}

pub struct Iter<T> {
    next: Link<T>,
}

impl<T> Iterator for Iter<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.clone().map(|node| {
            self.next = node.next.clone();
            node.elem.clone()
        })
    }
}


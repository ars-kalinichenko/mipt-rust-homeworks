#![no_std]

use core::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

pub struct SmolVec<T, const N: usize> {
    // TODO: your code goes here.
}

impl<T, const N: usize> SmolVec<T, N> {
    pub fn new() -> Self {
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

    pub fn push(&mut self, obj: T) -> Option<T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T, const N: usize> Default for SmolVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Index<usize> for SmolVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T, const N: usize> IndexMut<usize> for SmolVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // TODO: your code goes here.
        unimplemented!()
    }
}

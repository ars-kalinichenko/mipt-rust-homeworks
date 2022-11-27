#![forbid(unsafe_code)]

use crossbeam::channel::{unbounded, Receiver, Sender};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    thread,
};

////////////////////////////////////////////////////////////////////////////////

pub struct ThreadPool {
    // TODO: your code goes here.
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    // pub fn spawn(&self, task: ...) -> JoinHandle<...> {}

    pub fn shutdown(self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct JoinHandle<T> {
    // TODO: your code goes here.
}

#[derive(Debug)]
pub struct JoinError {}

impl<T> JoinHandle<T> {
    pub fn join(self) -> Result<T, JoinError> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

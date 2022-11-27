#![forbid(unsafe_code)]
use crate::{
    data::ObjectId,
    error::{Error, NotFoundError, Result},
    object::{Object, Schema, Store},
    storage::StorageTransaction,
};
use std::{
    any::{Any, TypeId},
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
    rc::Rc,
};

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.
pub struct Transaction<'a> {
    // TODO: your code goes here.
}

impl<'a> Transaction<'a> {
    pub(crate) fn new(inner: Box<dyn StorageTransaction + 'a>) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn ensure_table<T: Object>(&self) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn create<T: Object>(&self, src_obj: T) -> Result<Tx<'_, T>> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn get<T: Object>(&self, id: ObjectId) -> Result<Tx<'_, T>> {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn try_apply(&self) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn commit(self) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn rollback(self) -> Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjectState {
    Clean,
    Modified,
    Removed,
}

#[derive(Clone)]
pub struct Tx<'a, T> {
    // TODO: your code goes here.
}

impl<'a, T: Any> Tx<'a, T> {
    pub fn id(&self) -> ObjectId {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn state(&self) -> ObjectState {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn delete(self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

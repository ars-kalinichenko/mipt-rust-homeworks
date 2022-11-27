#![forbid(unsafe_code)]
use crate::hlist::{HCons, HNil};
use crate::labelled::LabelledGeneric;

pub enum Here {}
pub struct There<T>(std::marker::PhantomData<T>);

pub trait Plucker<Target, Indices> {
    type Remainder;
    fn pluck(self) -> (Target, Self::Remainder);
}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub trait Sculptor<Target, Indices> {
    type Remainder;
    fn sculpt(self) -> (Target, Self::Remainder);
}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub trait Transmogrifier<Dst, Indices> {
    fn transmogrify(self) -> Dst;
}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

// fn transform_from<Src, Dst, Indices>(src: Src) -> Dst

// TODO: your code goes here.

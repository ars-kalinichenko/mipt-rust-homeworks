#![forbid(unsafe_code)]
pub struct HNil;

pub struct HCons<H, T> {
    pub head: H,
    pub tail: T,
}

impl HNil {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HNil {
    fn default() -> Self {
        Self::new()
    }
}

impl<H, T> HCons<H, T> {
    pub fn new(head: H, tail: T) -> Self {
        Self { head, tail }
    }
}

////////////////////////////////////////////////////////////////////////////////

// macro_rules! HList
// macro_rules! hlist
// macro_rules! hlist_pat

// TODO: your code goes here.

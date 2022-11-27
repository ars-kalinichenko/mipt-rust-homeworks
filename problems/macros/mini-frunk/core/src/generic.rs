#![forbid(unsafe_code)]

pub trait Generic {
    type Repr;
    fn into(self) -> Self::Repr;
    fn from(repr: Self::Repr) -> Self;
}

// fn generic<Dst, Repr>(repr: Repr) -> Dst { ... }
// fn into_generic<Src, Repr>(src: Src) -> Repr { ... }
// fn convert_from<Src, Dst, Repr>(src: Src) -> Dst { ... }

// TODO: your code goes here.

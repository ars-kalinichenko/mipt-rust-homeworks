#![forbid(unsafe_code)]
pub trait LabelledGeneric {
    type Repr;
    fn into(self) -> Self::Repr;
    fn from(repr: Self::Repr) -> Self;
}

// fn from_labelled_generic<Dst, Repr>(repr: Repr) -> Dst
// fn into_labelled_generic<Src, Repr>(src: Src) -> Repr
// fn labelled_convert_from<Src, Dst, Repr>(src: Src) -> Dst

// TODO: your code goes here.

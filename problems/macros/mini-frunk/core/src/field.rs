#![forbid(unsafe_code)]

pub mod symbols {
    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

pub struct Field<N, T> {
    pub name_type_holder: std::marker::PhantomData<N>,
    pub value: T,
}

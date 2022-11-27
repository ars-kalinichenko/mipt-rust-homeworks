#![forbid(unsafe_code)]

pub struct LazyCycle<I>
where
    I: Iterator,
{
    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    // TODO: your code goes here.
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        // TODO: your code goes here.
        unimplemented!()
    }
}

// TODO: your code goes here.

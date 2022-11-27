# Itertools

In this problem, you'll write trait `ExtendedIterator` to extend the default `Iterator` trait.

## Task

Do the following:

- Make `ExtendedIterator` to be implemented automatically for any iterator.
- `ExtendedIterator::lazy_cycle()` - creates an iterator that repeats infinitely the input sequence of elements.
  - Underlying iterator should be exhausted lazily, not instantly after the call to the `lazy_cycle`, therefore `Item` must be `Clone`.
  - Remember that in Rust `None` does not necessarily mean iterator is exhausted, but in the case of `lazy_cycle` we should stop calling to the `.next()`.
- `ExtendedIterator::extract(index)` - gets n-th element (zero-indexed) from the iterator and returns the pair of n-th element and the new iterator that returns the same elements but without n-th one.
  - You can call `.next()` at most `index + 1` times.
  - You'll probably need to write `#[allow(clippy::needless_collect)]`: clippy is not ideal.
- `ExtendedIterator::tee()` - returns a pair of independent iterators that will return the same sequence of elements as the input one.
  - The idea is to clone elements inside, therefore we require `Clone` from `Item`.
  - Input iterator should be exhausted lazily, only when any of `Tee`'s gets to the element.
- `ExtendedIterator::group_by(func)` - combines all consecutive elements for which `func` returns the same value into groups. `group_by` returns an iterator over pairs, where the first element is `f(item)` and the second element is a `Vec` of consecutive elements such that `f(item)` for each of these elements is equal to the first element of the pair.

## Questions

- Why do we have to return structures like `Extract` instead of writing `impl Iterator` in trait? Think about it, then read the [RFC 1522](https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md#limitation-to-freeinherent-functions).
- Usually, instead of using `Tee` we just clone the iterator, but sometimes it's not clonable. It _may be_ a good idea to prevent using `Tee` when the iterator is `Clone`. But in stable Rust, it's not currently possible to negotiate trait bounds. Think why then read about [`negative_impls`](https://doc.rust-lang.org/beta/unstable-book/language-features/negative-impls.html) feature.

## Advanced level

- Make `extract` and `tee` work with iterators that don't end after first `None`.
- Implement `group_by` in such a way that it will return the pair of elements and the iterator over the group.

## Useful links

- [`.fuse()`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.fuse)
- [`chain()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)
- [`Itertools`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#) trait from the Itertools crate.

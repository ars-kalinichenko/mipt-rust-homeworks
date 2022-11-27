# FlatMap

In this problem, you'll implement a flat map - a data structure that stores key-value pairs in the sorted `Vec`.

## Implementation

Implement the methods of `FlatMap`. Note that functions that search by key (`get`, `remove`, `remove_entry`) must work not only with the key type but with any type `K: Borrow<Q>` and `Q: Ord + ?Sized`.

**Hint**: the signatures of these functions exactly match the signatures of the functions of the same name in `HashMap`.

Also implement the following traits:

- [Index](https://doc.rust-lang.org/std/ops/trait.Index.html)
- [Extend](https://doc.rust-lang.org/std/iter/trait.Extend.html) with `A = (K, V)`.
- [From](https://doc.rust-lang.org/std/convert/trait.From.html) from `Vec` and from `FlatMap`. If the vector has duplicates over keys, `FlatMap` should store only the last one.
- [FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) with `A = (K, V)`.
- [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)

## Benchmark

If you wish, you can benchmark your implementation using command `cargo criterion`.

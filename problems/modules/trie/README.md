# Trie

In this problem, you'll write your implementation of trie data structure and learn some of Rust's limitations. It's the bonus problem.

## Task

- Design trait `ToKeyIter`, which means the object can be used as a trie key. That means the object is iterable and iterator items are clonable. Here, you'll run into _Generic associated types_, since your associated types (iterators) will require lifetimes. Read [this blog](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html) to understand them.
- Write an actual trie implementation. You'll need the `Borrow` trait you're already familiar with. Since the borrowed type should have the same `Item` type in `KeyIter`, you'll need to compare these types. It's called [equality constraints](https://github.com/rust-lang/rust/issues/20041), but you actually don't need them even in the stable! Find a way to solve this issue.

## Notes

- This problem uses a **nightly** toolchain. Install it by using the command `rustup update nightly` and then run `cargo +nightly test` or just use `rover test`.
- The performance of binary in this problem **dramatically** increases in release build. When you'll be running stress or performance tests, use the `--release` flag.

## Questions

- Imagine we've solved the problem [without using GATs](https://stackoverflow.com/questions/33734640/how-do-i-specify-lifetime-parameters-in-an-associated-type). How this code will work? (Tip: the tests even won't compile)
- When we should choose generics over associated types and vice versa?
- Find the possible maximum of mistakes in the API design of this problem. Remember: users cannot use good API in the wrong way.

## Bonus

Design traits `FromKeyIter` and `TrieKey`.

- `FromKeyIter` means this iterator could be collected to the instance of the type it was created from.
- `TrieKey` means that this type could be turned into an iterator for trie and then recreated from this iterator.

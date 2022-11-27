# Binary search tree

In this problem, you'll write a binary search tree with a feature to find an n-th element.

Tests are quite heavy, don't forget to use `--release` builds!

## Task

Implement a BST with a default interface for collections you're already familiar with. You can use any perfect balancing algorithm (i.e algorithm that gives stable `O(logN)` asymptotic), but AVL is highly recommended since it's the easiest one.

## Notes

- If you have some troubles implementing rotations in-place with mutable references, use owned types instead. For instance:

  ```rust
  fn rotate_left(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>>
  ```

  Should take a root and return a new rotated root. The method [`.take()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.take) of an `Option` is your best friend.
- Try to write your safe implementation as performant as C or C++ one, without additional moves. Remember that checks for "null pointer" don't actually make your code much slower on modern CPUs.

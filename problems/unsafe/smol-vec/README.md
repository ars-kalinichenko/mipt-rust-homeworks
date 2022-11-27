# SmolVec

> _- Explain your smolness_
>
> _- Am on stack_

In this problem, you'll write a vector which size is limited and all the content is located on the stack.

## Preparation

You'll need unsafe Rust to solve this problem. Before solving, [install Miri](../../../docs/additional-setup.md#miri-interpreter).

## Implementation

You need to implement the part of API of `Vec`. It's basically the same with the only difference in `push`: since our vector have limited capacity and no chance to grow, we'll need to return the element we're pushing if there's no free space. You're also limited to `no_std` Rust.

## Note

In practice, consider using one of the following:

- [arrayvec](https://crates.io/crates/arrayvec) - stores on the stack only.
- [smallvec](https://crates.io/crates/smallvec) - stores some elements on the stack, falls back to the heap if the stack capacity is exceeded.
- [tinyvec](https://crates.io/crates/tinyvec) - stores some elements on the stack, falls back to the heap if the stack capacity is exceeded. 100% Safe Rust!

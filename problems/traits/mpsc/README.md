# Single thread MPSC-channel

In this problem, you'll implement a simple Multi Producer Single Consumer channel that works only in one thread.

## Task

In Rust, MPSC-channel consists of two objects: `Sender` and `Receiver`. `Sender` is a clonable type since we have a Multi Producer channel, and therefore `Receiver` is unique.

`Receiver` has `.close` function that closes the channel making `Sender`'s unable to write to it. The drop of `Receiver` or all `Sender` also closes the channel.

## Tips

- `Sender` and `Receiver` will have a sharable state - queue buffer with written elements. Since we don't know `Sender`'s or `Receiver` will drop faster, this buffer should be stored in reference counting pointer - `Rc`.
- Use `RefCell` to mutate the buffer that is located inside `Rc`.
- If you're interested, you can check [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/) module of the standard library that contains multithreaded MPSC-channel.

# File system walker

In this problem, you'll implement a simple file system traversal library.

## Task

The public interface of the library consists of the `Walker` structure, which has two important methods:

- `add_callback` - add a callback function that will be called during the traversal.
- `walk` - walk the specified path.

The callback takes a single argument, `handle`. There are 3 variants of handle:

- `dir` - matches the directory encountered during the crawl. If the callback calls `.descend()` to `dir`, then the traversal will go deeper into that directory.
- `file` - matches the file encountered during the crawl. If the callback calls `.read()` on `file`, then the contents of this file will be read and passed to the callback in one of the following calls.
- `content` - the content of the file.

There can be many callbacks in the same `Walker`. If some callback didn't call `.descend()` on a directory, it must not be called on the contents of that directory. At the same time, other callbacks could express a desire to bypass this directory, so `Walker` must descend into it anyway.

## Implementation

Implement a simple recursive filesystem traversal algorithm:

1. Make a list of files in the current path;
2. For each entry of the current path:
    - If it's a file:
        1. Call each callback for this file;
        2. If there is at least one callback interested in reading this file, read the file and call all interested callbacks on its contents.
    - If it's a directory:
        1. Call each callback to the given directory;
        2. If there is at least one callback interested in bypassing this directory, call recursively to this directory with a list of descended callbacks.

If an I/O error occurs during the traversal, discard the branch where the error occurred, but don't stop the traversal. As a result of calling `walk`, return any of the errors encountered, or `Ok(())` if there were no errors.

The callback order is **not specified**, it can even change during the traversal!

## Hints

- When recursively calling the traversal function, it is convenient to pass interested callbacks as `&mut [&mut Callback]`. You can do a partition of this list of callbacks by criteria "whether this callback called `descend`", and walk a subdirectory with a subslice (`&mut callbacks[..index]`).
- For partitioning in `std`, there are functions [partition_in_place](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition_in_place) (unstable) and [partition](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition). They won't work anyway, because they take a constant reference to the element (and you need a `&mut` reference to the callback to call `FnMut`). You'll have to implement your partition with the correct signature.

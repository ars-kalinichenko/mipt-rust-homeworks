# Parallel `grep`

In this problem, you'll implement a simple parallel `grep`.

## Interface

The function `run(path, pattern)` should perform a recursive search at the specified path and all of its subpaths for occurrences of the specified substring `pattern`. It returns a list of events, where the event can be either a found occurrence of substring or an I/O error. More specifically:

- `Event::Match` represents success. Contains the path to the file, the line number in file and the line itself.
- `Event::Error` represents an I/O error. Contains the path to the file where the error occurred and the error itself.

Tests also check whether your implementation is at least as fast as naive one-threaded.

## Tips

- Search parallelism is achieved by processing different files in different threads. Additionally, you can try to start processing files _alongside_ with the traversal of the filesystem.
- In this problem, you should use the `rayon` crate. Perfect solution should use it just once by calling `.par_iter()` or `.par_bridge()` on some iterator.

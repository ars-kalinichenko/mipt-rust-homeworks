# Thread pool

In this problem, you'll write a simple thread pool.

## Interface

- The `.new` method accepts the number of threads.
- The `.spawn()` method takes a task, that is actually a function, and returns its handle.
- The `JoinHandle` that is returned from `.spawn()` has a `.join()` method that returns `Result<T, JoinError>`. Here `T` is the type of the value that the task returns. `JoinError` is returned if the task panics.
- The `.shutdown()` method of the thread pool waits until all current threads finish their work and terminate them.

## Implementation

- We won't support variables from the scope in our thread pool, so all input types must be `'static`.
- To stop stack unwinding in case of a panic, use:

    ```rust
    use std::panic::{catch_unwind, AssertUnwindSafe};

    catch_unwind(AssertUnwindSafe(task))
    ```

- Use `crossbeam::channel::unbounded()` to distribute tasks across threads.
- If you know what condition variables are, you might want to use them. Whereas this may be faster, it will complicate the code a lot. Prefer writing simple code.

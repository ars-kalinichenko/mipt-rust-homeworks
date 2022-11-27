# Queue with minimum

Implement a queue that can also tell what the current minimum is. **The implementation must have at least amortized O(1) complexity for all calls.**

## Tips

- If you don't remember how this queue is working, read it on [cp-algorithms](https://cp-algorithms.com/data_structures/stack_queue_modification.html).
- You'll need to use [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) to solve this problem. Check the methods [`push_back`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.push_back), [`pop_front`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_front), [`pop_back`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.pop_back), [`front`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.front).

# LRU Cache

In this problem, you'll write a cache with [Least Recently Used](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU) strategy.

## Task

Implement the LRUCache structure:

- `LRUCache::new(capacity: usize) -> Self` - Initialize the LRU cache with some _positive_ size capacity.
- `LRUCache::get(&mut self, key: &K) -> Some(&V)` - Return the `Some(&value)` of the key if the key exists, otherwise return `None`.
- `LRUCache::insert(&mut self, key: K, value: V) -> Some(V)` - Update the value of the key and return `Some(value)` with old value if the key exists. Otherwise, add the key-value pair to the cache and return `None`. If the number of keys exceeds the capacity of this operation, evict the least recently used key.

You're not required to write the best O(1) implementation and for that `K` is clonable, hashable and comparable. The last two mean you can use `BTreeMap` and `HashMap`. However, they have to be something like O(log(N)) at least to pass the stress test. Check this test out!

## Questions

- Is there a way to implement it without `Clone` on `K`?
- Is it possible to get pure O(1) without unsafe?

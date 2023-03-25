# Context

In this problem, you'll create a structure `Context` that stores objects of any type.

## Task

First, read about the trait [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html).

Implement the following interface:

- `Context::new()` - creates new empty context.
- `Context::insert::<T>(key, value)` - add a key-value pair to the context, where the key is some string type, and value can be any type. If the context already has the value, it's overwritten.
- `Context::get(key)` - gets a value and returns it as type `T`. If there's no such key, panic.
- `Context::insert_singletone(obj)` - adds an object of type `T` to the context. If the object is already present, it's replaced by a new value.
- `Context::get_singletone::<T>()` - gets an object of type `T` from context, added previously by `insert_singletone`. If there isn't such an object, panic.

## Questions

- Why do we require `get` to provide a type `T`?

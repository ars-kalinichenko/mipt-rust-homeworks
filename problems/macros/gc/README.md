# Garbage collector

In this problem, you'll write a "garbage collector" for Rust.

## Task

Our simple garbage collector API will consist of two types:

- `Gc<T>` is a smart pointer pointing to an object owned by the garbage collector. To get a reference to `T`, the user calls `gc.borrow()`.
- `Arena` - creates a new `Gc<T>` and keeps track of them. You can assume that all `Gc<T>`'s are created by just one arena. When the user wants to release all unused objects, he calls `arena.sweep()`.

Type `T` must be able to understand to what objects it refers to. For instance, `Vec` refers to multiple objects, and each object can have a `Gc` inside. That means we should implement some trait (we'll name it `Scan`) to do actual reference counting. You need to come up with the contents of the trait yourself, and also implement derive macro for `Scan`.

## Algorithm

Let `Arena` contain strong pointers (`Rc`) to all objects allocated in it, and `Gc<T>` contains a weak pointer (`Weak`). Then objects cannot be in a cycle of a strong pointers, and in order to delete the object, it is enough delete strong link to it.

Note that `gc.borrow()` returns `GcRef<'a, T>` which contains a reference to the parent `Gc<T>`. `GcRef`, of course, contains strong pointer; but the reference captures the invariant that for any `GcRef` there will always be a live `Gc`. We will use this property later.

Therefore, the garbage collection algorithm look like this:

1. Count for each object the number of other objects that refer to it (have `Gc<T>` on it).
2. Check for each object its weak reference counter. If the count is greater than than the number from step 1 for this object, this means that there's an external `Gc<T>`, which refers to this object. Let's mark such objects.
3. For each object from step 2, mark all objects that are reachable from this object (i.e. objects that can be reached by inner `Gc<T>`).
4. All objects not marked in step 3 are unreachable for code from outside of the arena. We'll delete such an objects.

Note that in step 4 all unmarked objects are guaranteed to have only one strong the pointer - the one owned by the arena. If not, then the object has an external `GcRef`, but then there is an also an external `Gc`, which means that an object cannot be a candidate for deletion.

You can notice that nothing prevents the user from storing a `GcRef` inside a garbage collected object. We'll assume that our derive macro will refuse to derive `Scan` for the types with field `GcRef`. This behavior is not tested.

## Implementation

- Note that a garbage collected object may contain inside both "bare" `Gc` on other objects, as well as `Option<Gc<T>>`, `Vec<Gc<T>>`, `RefCell<Gc<T>>` and their arbitrary combinations. You don't have to generate specific logic for each type inside the derive macro; better write blanket implementations of the Scan trait like this:

  ```rust
  impl<T: Scan> Scan for Option<T> {
      // ...
  }
  ```

- Before writing derive macro, it is better to implement Scan for the types you need directly in the file `tests/tests.rs`. When the tests pass, start writing a macro in `gc-derive/src/lib.rs`.
- When traversing an object graph, it's convenient to work with the addresses of these objects as `usize`. To get an address as `usize` from `Gc<T>`, do `self.weak.as_ptr() as usize`.

## Questions

- What will happen if `Arena` is dropped before the `Gc`'s?
- What will happen we've allowed multiple arenas?

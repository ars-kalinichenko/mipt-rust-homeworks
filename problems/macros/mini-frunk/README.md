# Mini `frunk`

In this problem, you'll implement the small part of the [`frunk`](https://crates.io/crates/frunk/) crate for structural typing. It's the bonus problem.

## Motivation

Imagine you're processing JSONs in your service. Eventually, you'll want to transform one JSON into another. The only alternative we currently heard of is the `serde` crate, which enables us to serialize and deserialize types.

```json
}
    "user_id": "john_doe",
    "items": ["salad", "juice", "beer", "fork"],
    "version": 0,
}
```

```rust
// Actual JSON structure
#[derive(Deserialize)]
struct Json {
    user_id: String,
    items: Vec<String>,
    version: u64,
}
```

...And then we're defining all of the needed combinations of fields:

```rust
struct VersionAndUser {
    version: u64,
    user: String,
}
struct VersionAndItems {
    version: u64,
    items: Vec<String>,
}
struct UserAndItems {
    user: String,
    items: Vec<String>,
}

// Creating more JSONs
let deserialized: Json = serde_json::from_str(&str_with_json).unwrap();
let version_and_user = VersionAndUser {
    version: deserialized.version,
    user: deserialized.user.clone(),
}
let version_and_items = VersionAndItems {
    version: deserialized.version,
    items: deserialized.items.clone(),
}
let user_and_items = UserAndItems {
    user: deserialized.user.clone(),
    items: deserialized.items.clone(),
}
```

What if I'll tell you that you can make all of these transformations in **one-line** using the `frunk` crate?

```rust
let version_and_user: VersionAndUser = deserialized.transmogrify();
```

Let's find out how it works!

**Note**: This problem will look like _"I should've written boilerplate instead..."_ after some time. It's challenging, don't solve it if you don't feel ready. And consider using already implemented `frunk` in production code.

## Idea

1. First, read the definitions of `HNil` and `HCons` from `core/src/hlist.rs`. These two enable us to write an _infinite tuple_: for instance, `HCons<u32, HCons<String, HCons<Vec<i32>, HNil>>>` is actually `(u32, String, Vec<i32>)` even in binary. This idea is called _a heterogeneous list_.
2. Find out that we can actually represent two structures with the same field types in the same order as a heterogeneous list! For instance, these three are actually identical and can be represented as `HCons<&'a str, HCons<&'a str, HCons<usize, HNil>>>`:

    ```rust
    pub struct Strategist<'a> {
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub age: usize,
    }
    pub struct President<'a> {
        pub first_name: &'a str,
        pub last_name: &'a str,
        pub age: usize,
    }
    pub struct JumbledPerson<'a> {
        pub last_name: &'a str,
        pub first_name: &'a str,
        pub age: usize
    }
    ```

    So, any structure can be represented as a heterogeneous list.

3. Let's define a trait called `Generic`:

    ```rust
    pub trait Generic {
        type Repr;
        fn into(self) -> Self::Repr;
        fn from(repr: Self::Repr) -> Self;
    }
    ```

    `Repr` will contain the actual structure representation in a heterogeneous list, just like `Strategist`, `President` and `JumbledPerson` have. The `.into(self)` function will consume the structure and return a variable of type `Repr`. The `.from()` function will do vice versa.

4. Of course, we need a derive macro for `Generic`! We don't want to write these types by hand.
5. As you can see, `Strategist` and `JumbledPerson` have the same `Repr`! This means that using `Generic` we can convert one from another and produce the correct result in runtime:

    ```rust
    let person: Person = from_generic(hlist!("Humpty", "Drumpty", 3));
    let jumbled_person: JumbledPerson = convert_from(person.clone());
    assert_eq!(person.first_name, jumbled_person.last_name);
    ```

    We want to check not only the types of fields but also the **names**.

6. First of all, we'll represent names in type level:

    ```rust
    // Symbols in names
    enum a {}
    enum b {}
    enum c {}
    // ...
    enum A {}
    // ...
    enum __ {}

    // This is how 'first_name' field is represented
    (f, i, r, s, t, __, n, a, m, e)
    ```

    We'll create a `Field` structure representing a field of the structure:

    ```rust
    pub struct Field<N, T> {
        pub name_type_holder: std::marker::PhantomData<N>,
        pub value: T,
    }
    ```

    [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) is used when we want to depend on type and its limitations but don't actually store it.

7. Let's create a new trait called `LabelledGeneric`:

    ```rust
    pub trait Generic {
        type Repr;
        fn into(self) -> Self::Repr;
        fn from(repr: Self::Repr) -> Self;
    }
    ```

    `Repr` here is more complicated. For instance, `Strategist` has:

    ```rust
    HCons<
        Field<(f, i, r, s, t, __, n, a, m, e), &'a str>,
        HCons<
            Field<(l, a, s, t, __, n, a, m, e), &'a str>,
            HCons<
                Field<(a, g, e), usize>,
                HNil,
            >,
        >,
    >;
    ```

    So, every type can be represented at the type level by using a heterogeneous list and names on the type level.

8. Of course, we need a derive macro for `LabelledGeneric`! We don't want to write these types by hand.
9. Our example with `Strategist` and `JumbledPerson` now will fail due to different `Repr` types. But we want to have a way to transform them!
10. First, let's define a trait called `Plucker` that will enable us to extract the first field of the selected type:

    ```rust
    // To understand what's Indices, read Implementation section
    pub trait Plucker<Target, Indices> {
        type Remainder;
        fn pluck(self) -> (Target, Self::Remainder);
    }
    ```

    Now we can do this:

    ```rust
    let hlist = hlist![1, "hello", true, 42f32];
    let (t, r): (f32, _) = hlist.pluck();
    assert_eq!(t, 42f32);
    assert_eq!(r, hlist![1, "hello", true])
    ```

11. Since we already can extract types, we are ready to finally implement `Sculptor`:

    ```rust
    // To understand what's Indices, read Implementation section
    pub trait Sculptor<Target, Indices> {
        type Remainder;
        fn sculpt(self) -> (Target, Self::Remainder);
    }
    ```

    `Sculptor` will use `Plucker` to extract types one by one and build the new list from the old one. Now we can do this:

    ```rust
    let hlist = hlist![9000, "joe", 41f32, true];
    let (reshaped, remainder): (HList![f32, i32, &str], _) = hlist.sculpt();
    assert_eq!(reshaped, hlist![41f32, 9000, "joe"]);
    assert_eq!(remainder, hlist![true]);
    ```

12. Implement `Transmogrifier` for all `LabelledGeneric` types. This one will just sculpt one list to another!

    ```rust
    // To understand what's Indices, read Implementation section
    pub trait Transmogrifier<Dst, Indices> {
        fn transmogrify(self) -> Dst;
    }
    ```

## Implementation

It's recommended to read tests to understand what's required.

1. Implement macros `HList!`, `hlist!` and `hlist_pat!` for creating lists of types. The first one returns a list of types using structures `HNil` and `HCons<H, T>`, whereas the second returns an actual list with values, and the third will be used to match. Pass the tests in `hlist_macro.rs`. Try to reuse them as much as you can later.
2. Implement the derive macro for trait `Generic`. Define and implement `from_generic`. `into_generic` and `convert_from` functions. Read tests in the file `generic.rs` with what structures your macro should work.
3. Implement the derive macro for the trait `LabelledGeneric`. Define and implement `from_labelled_generic`, `into_labelled_generic`, `labelled_convert_from` functions. Read tests in the file `generic.rs` with what structures your macro should work.
4. Implement traits `Plucker`, `Sculptor`, and `Transmogrifier`.

    When `HCons<Head, Tail>` implements `Plucker<Target, _>`, this means there's a type `Target` inside the list. Base of induction: `Head = Target`. Step: we `pluck` the `Target` from the `Tail` and then return the target and `Cons<Head, *Tail remainder*>`.

    Why do we need `Indices`? Consider the situation when both `Target = Head` and `Tail: Plucker<Target>`. The compiler won't know what implementation to choose!

    So, we'll add another argument called `Indices` to represent the path to the target. New base of induction: `Head = Target` and `Indices = Here`. Step: we are generic over `TailIndices` and implement `Plucker<Target, There<TailIndices>> for HCons<Head, Tail>`. Since we are implementing `Plucker` with different generics, it's different implementations and the compiler won't be angry!

    The same applies to `Sculptor` and `Transmogrifier`.

Just notable: all of this code will be optimized to **zero-cost abstraction**!

## Links

- [Rust Generic (Not Generics)](https://beachape.com/blog/2017/02/04/rust-generic-not-generics/) - About `Generic`. From the author of `frunk`.
- [LabelledGeneric in Rust: What, Why, How?](https://beachape.com/blog/2017/03/04/labelledgeneric-in-rust-what-why-how/) - About `LabelledGeneric`.
- [Gentle Intro to Type-level Recursion in Rust: From Zero to HList Sculpting](https://beachape.com/blog/2017/03/12/gentle-intro-to-type-level-recursion-in-Rust-from-zero-to-frunk-hlist-sculpting/#plucking-from-hlists) - About `Plucker`.
- [Boilerplate-free Struct Transforms in Rust](https://beachape.com/blog/2017/04/12/boilerplate-free-struct-transforms-in-rust/) - How `transmogrify` works.
- [(Russian) Heterogeneous lists in Rust and their superpowers](https://www.youtube.com/watch?v=Zps2tH8XOm4&list=PLRdS-n5seLRroZ480sDtes06hn6_M7N_i&index=6) - The exact thing we're writing.

## Notes

- You'll want to create `enum`'s for different symbols for possible JSON names. Don't just repeat the definitions, try to use macro here!

## Questions

- Why not just write a macro that will convert one structure to another?
- Why do we frequently use empty `enum`'s?
- In the actual implementation of `Field` in `frunk` there's `pub name: &'static str` field. When it's used?
- Read about quite a famous [`specialization`](https://rust-lang.github.io/rfcs/1210-impl-specialization.html) feature from the nightly compiler. Try to write `Plucker` using it. Why there are conflicting implementations? Can we somehow implement it with specialization?

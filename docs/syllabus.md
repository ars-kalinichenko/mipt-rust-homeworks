# Course syllabus

This document is the course syllabus. It contains the grading and lecture course program.

## Grading

- You have to solve all of the obligatory tasks to achieve a Satisfactory mark. Moreover, it's enough for a Satisfactory mark.
- Every task has a score X. After the deadline (usually 3 weeks) the score linearly lowers from X to X/2.
- There are 3 projects in the semester. Every project costs 1 point. After the project deadline (usually 4 weeks) the score linearly lowers from 1 to 1/2 for 2 weeks. Your maximum mark is determined by _ceil_ value of these points:
  - 0 points - Satisfactory(4) is maximum;
  - 1 point - Good(7) is maximum;
  - 2 points - Excellent(9) is maximum;
  - 3 points - Excellent(10) is maximum.
- Let `project_max()` be the function, which returns your maximum mark (4, 7, 9 or 10) depending on project points, `my_score()` the sum of your scores, and `max_score()` the sum of all scores without deadline penalties. If all of the obligatory problems are solved, then your mark is:
  - `min(project_max(), max(3, round(10 * my_score() / max_score())))`

## Lectures program

1. Hello, Rust! Discussing the language. Comparison with C++. Basics of language.
    - Why do we even need Rust? What problems does Rust solve? Where is Rust intended to be used? What companies are already using Rust?
    - What do _safe_ and _unsafe_ mean. What do _sound_ and _unsound_ mean. What Rust considers safe. RustBelt formal model.
    - `let` and `mut` keywords. Data types `i8...i128`, `u8...u128`, `isize`, `usize`, `f32`, `f64`, `bool`. Literals.
    - Shadowing. Type inference.
    - `as` keyword for primitive casts. Non-transitivity of casting.
    - Compound types `array` and `tuple`. Unit type `()`.
    - `T`, `&mut T`, `&T`.
    - Remind of C++ `std::reference_wrapper`.
    - Functions. `return` keyword. Expressions and statements.
    - `struct`. Functional update. `self` and `Self`. `impl` keyword. Associated functions. Introduction to generics.
    - `enum`. `size_of` `enum` and discriminant. `std::cmp::Ordering`. Comparing with C++'s `enum` and `union`. Meaning of `_`.
    - Syntax of `if`, `while`, `for`, `loop`. Named `break`. `if`/`while` and `let` syntax sugar.
    - `match`. Matching ranges. Multiple patterns. Binding modes. Destructuring.
    - Introduction to `Vec` and part of its interface. Slice `&[T]`.
    - `panic!`, `unimplemented!`, `unreachable!`, `todo!`, `println!`, `assert!`, `assert_eq!`.
    - Inhabited type `!`.
    - Borrow checker and how not to give up at the beginning. The borrowing mechanism.
    - Introduction to Rust ecosystem. VS Code plugins: official and rust-analyzer.
    - `std::prelude`. Basic usage of `cargo` and `rustc`.

2. Standard library types and collections.
    - `Vec`, `VecDeque` and their interface.
    - `HashMap`, `HashSet` and their interface. Asymptotics.
    - `BTreeMap`, `BTreeSet` and their interface. Why can't B-Tree be used like `std::map`.
    - `LinkedList`, `BinaryHeap` and their interface.
    - `String`. Random access. UTF-8. `&str` and reminder of C++ `string_view`. `char` and meaning of Unicode scalar value.
    - `Result` and its interface.
    - `Option`, its interface and compiler optimizations.
    - Heap allocations: `Box`, `Rc`, `Cow` and their interface. Mention of `Arc`.
    - Why `Rc` is immutable.
    - Module `std::cell`. Interior mutability. `Cell`, `RefCell`. Reentrancy.
    - `std::mem` module and it's safe features: `size_of`, `swap`, `replace`, `forget`, `drop`.
    - Drop checker basics. Drop flags. Stability of drop order and reasons. Initialization order.
    - Exotically sized types: ZST, DST, Empty. Containers, especially `Vec`, when `T` is ZST.
    - `NonNull`, `NonZero`.
    - `print!`, `println!`, `eprint!`, `eprintln!`, `write!`, `writeln!` and locking of IO stream.
    - `BufReader` and `BufWriter`. Their interface and reducing allocations count.

3. Traits. Functional programming features. Iterators.
    - Traits. Return type polymorphism. Auto traits. `where` keyword. Extension traits.
    - Basic library traits and their methods. `Default`, `Clone`, `Copy`. Why aren't they derived by default?
    - `Ord`, `PartialOrd`. `Eq`, `PartialEq`. `Hash`, `Hasher`. `Drop`, `ManuallyDrop`, and RAII. Relying on drop order.
    - Module `std::ops`. Traits `Add`, `Sub`, `Mul`, `Div`, `Rem`, `BitAnd`, `BitOr`, `BitXor`, `Shl`, `Shr` and their `-Assign` variants. `Not`, `Neg`.
    - Traits `Index` and `IndexMut`.
    - Traits `Debug` and `Display`. `Formatter`. The motivation of their design. Trait `ToString`.
    - `Deref`, `DerefMut`, `Borrow`.
    - Module `std::convert`. Traits `From` and `Into`, `TryFrom` and `TryInto`, `AsRef` and `AsMut`. `identity` function.
    - Compound types `array` and `tuple`: what changes when their size is big.
    - Associated types and consts.
    - Iterators. Laziness of iterators. Traits `Iterator`, `IntoIterator`. Implementation of iterators in `std`. Iterators in Runtime.
    - API of iterators: `map`, `filter`, `fold`, `flatten` and others.
    - Motivation of `Iterator` trait design.
    - Iterator invalidation in C++ and Rust.
    - Iterators and vectorization. How to return iterator and closure from function: `impl` keyword.
    - Module `std::iter` useful functions: `from_fn`, `empty`, `once`, `repeat`, `repeat_with`.
    - Traits `FromIterator`, `ExactSizeIterator`, `DoubleEndedIterator`, `Index`, `IndexMut`.
    - `collect`, `flatten` and their implementation.
    - Traits `FnOnce`, `Fn`, `FnMut`. Closures. Capture clause. `move` keyword. Variable rebinding in a separate scope.

4. The Rust language package manager: Cargo. Error handling.
    - Cargo. Crates and modules. Compilation unit. What's in a crate. Coherence.
    - Cargo package structure. Cargo.lock, semantic versioning. Rustup. [crates.io](https://crates.io). Types of library crates.
    - `use`, `mod`, `pub`, `super`, `crate`. Where `pub` doesn't work.
    - Rust release cycle. Raw identifiers. Migrating to other edition.
    - Error handling. Recoverable and unrecoverable errors. Panic and stack unwinding. Unwind safety. `Result<T, E>`. Operator `?` and deprecated `try!`. Best practices of error handling.
    - Exception safety: minimal and maximal.
    - Controlling panic. `catch_unwind`, `resume_unwind`.
    - `Error` trait and its problems.
    - Basics of crates anyhow and thiserror.

5. Metaprogramming in Rust. Writing idiomatic code.
    - Generics. Monomorphization. Static and dynamic polymorphism.
    - Trait specialization.
    - Reason why there's no generics partial specialization: ugly SFINAE consequences.
    - Macro. `macro_rules!`. Patterns, `$crate`. Identificators. Hygiene. Macros problems. Internal macro.
    - Basics of crate serde.
    - Attributes. `non_exhaustive`, `deprecated`. Macro `env!`, `option_env!`, `stringify!`, `include_str!`.
    - Conditional compilation and crate cfg-if.
    - Procedural macro. `derive`, `cfg`, `test`. `recursion_limit` attribute for macros.
    - Basics of crate syn.
    - Metaprogramming. Constant evaluation and `const` keyword. Const generics. Macro code generation.
    - Software design patterns: command, interpreter, newtype idiom, strategy, visitor, builder, fold.
    - Software design antipattern: using deref polymorphysm.
    - Tips of writing idiomatic code.

6. Virtual method table. Memory management and roots of system safety.
    - Virtual method table. Fat pointer. Keyword `dyn`. Dynamic dispatch. On-Stack Dynamic Dispatch. Dynamically sized types.
    - Module `std::any`. Trait `Any`.
    - Type coercion and subtyping. Fully Qualified Syntax and when to use it.
    - Object Safety. Generics in the virtual table. `Hash` and inline.
    - How Rust manages memory: aliasing and "Aliasing XOR Mutability" rule (AXM).
    - Borrow checker, affine type system.
    - Lifetimes. Named references. Lifetime elision. Reborrowing.
    - Unbounded `'static` lifetime: why do we need it and what's relationship to other lifetimes.
    - Higher-Rank Trait Bounds (HRTB). Variance.
    - `ref` keyword and `match`. Two phase borrows.
    - Drop checker. Connection of `PhantomData` and variance inference.
    - The dot operator and rules of auto dereference.

7. Working with filesystem. Roots of system safety: RustBelt research.
    - Working with file system with module `std::fs`. Comparing the design of Rust and Go.
    - GhostCell paper. Discussing the power of Rust type system and static checking.
    - Aliasing model. Stacked borrows paper.

8. Automatic Rust verification and support tools.
    - [Clippy](https://github.com/rust-lang/rust-clippy) and its [lints](https://rust-lang.github.io/rust-clippy/master/index.html).
    - [Rust analyzer](https://github.com/rust-analyzer/rust-analyzer).
    - MIR interpreter [Miri](https://github.com/rust-lang/miri).
    - [Rudra](https://github.com/sslab-gatech/Rudra) static analyzer.
    - Dynamic Symbolic Execution (DSE) tools: [Rust verification tools](https://github.com/project-oak/rust-verification-tools) (RVT), [Cargo-KLEE](https://gitlab.henriktjader.com/pln/cargo-klee).
    - Model checkers: [Rust Model Checker](https://github.com/model-checking/rmc) (RMC), [SMACK](https://github.com/smackers/smack)
    - Verification tools: [Haybale](https://github.com/PLSysSec/haybale), [Stateright](https://github.com/stateright/stateright).

9. Parallel and concurrent computing in Rust.
    - Rust memory model state. Orderings. Connection of memory safety and absence of data races.
    - Module `std::thread`.
    - Threads. Thread builder. Scope and static. `thread::scoped` and its problems. Closure scope. Panic in closure scope.
    - Basics of crates rayon and crossbeam.
    - `Send` and `Sync` traits. Unsafe traits. `Ord` and undefined behavior.
    - `std::atomic`: `Atomic` and `fence`.
    - `Arc` and example of undefined behavior in unsafe code. `Mutex` and its poisoning. `RwLock`. `Lazy`.

10. Asynchronous Rust and networking.
    - Motivation of asynchronous Rust design.
    - `async` and `await`.
    - Stackless coroutines.
    - Trait `Future`. Building an executor.
    - `Pin`, `Unpin` and their use cases. `PhantomPinned`.
    - Basics of different networking protocols from the ground up.
    - Open Systems Interconnect model (OSI). Reminder of Ethernet, IP, UDP and TCP.
    - Networking in Rust. Module `std::net`. `IpAddr`. `TcpListener`, `TcpStream` and `UdpSocket`.
    - Collecting metrics in Rust.
    - Basics of crates tokio and loom.

11. Unsafe Rust. Representation of types in memory.
    - `unsafe` keyword. The contract between safe and unsafe code. What unsafe can do. When do we need unsafe. Problem: safe `mem::forget`. `mem::transmute`. `static` keyword. `UnsafeCell`. Pointers: `*const T`, `*mut T`.
    - `MaybeUninit`. Compiler optimizations, `Container<MaybeUninit<T>>>` and `Container<T>`.
    - Unsafe traits. Problems with safety on `BTreeMap` and trait `Ord`.
    - WTF-8. `PathBuf` and `Path`.
    - `CString` and `CStr`. `OsString` and `OsStr`.
    - Basics of crates `cbindgen` and `rust-bindgen`.
    - Drop checker. `may_dangle` attribute.
    - Implementation of `split_at_mut`.
    - Discussing the implementation of `Vec`.
    - Discussing the implementation of `Arc` and `Mutex`.
    - Module `std::alloc`. Functions `alloc`, `alloc_zeroed`, `dealloc`, `Layout`. `GlobalAlloc`.
    - `realloc` in Rust and `C++`. How `move` works in Rust and C++. Move failures in C++. Discussing the moveit crate. Copying and cloning in Rust.
    - Leaking. Discussing the once_cell crate.

12. Rust and interacting with system and other languages.
    - Sections `.data`, `.rodata`, `.bss` and `.text`. Structure of heap and stack. Buffer overflow.
    - Representation of types in memory. Exotically sized type Extern. `enum` in memory. Guaranties of `Option`. Data layouts: `C`, `transparent`, `u*`, `i*`, `packed`, `align(n)`. Field ordering.
    - Using C/C++ code from Rust. Using Rust code from C/C++.
    - Rust ABI state.
    - Tips for writing FFI.
    - Signal handling.
    - Calling to the kernel.
    - The state of Rust in Linux kernel.

13. Techniques of speeding up Rust.
    - Module `core::arch` and SIMD. `core::intrisics` and its usage in standard library.
    - Module `std::hint`.
    - Link-time Optimization (LTO).
    - Profile-guided Optimization (PGO).
    - Rust profiling tools.
    - Code inlining. Reasons and examples.
    - `dyn` vs `impl` when both can be used.
    - Note about standard libraries `HashSet` and `HashMap`. Siphash and alternatives.
    - Removing and reducing the number of heap allocations and reallocations. Profiling `malloc` and `free`.
    - Short vectors problem. Discussing the smallvec and arrayvec crates.
    - Speeding up compilation and linking of big Rust projects.

### [Additional reading](reading-list.md)
